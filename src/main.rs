mod art;
mod audio;
mod game;
#[cfg(feature = "smoke-test")]
mod smoke;
mod ui;

use game::{Event, Game, Screen};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Barkely's Highlands Adventure".into(),
        window_width: 1280,
        window_height: 900,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let smoke_requested = args.iter().any(|a| a == "--smoke");
    #[cfg(not(feature = "smoke-test"))]
    if smoke_requested {
        eprintln!("Build with --features smoke-test to run the graphical smoke test.");
        return;
    }
    let path = if smoke_requested {
        Some(std::path::PathBuf::from("/tmp/barkely-smoke/progress.json"))
    } else {
        None
    };
    let mut game = Game::new(path);
    let audio = audio::Audio::new().await;
    let target = render_target(art::W as u32, art::H as u32);
    target.texture.set_filter(FilterMode::Linear);
    #[cfg(feature = "smoke-test")]
    let mut smoke = if smoke_requested {
        Some(smoke::Smoke::new(&mut game))
    } else {
        None
    };
    #[cfg(feature = "smoke-test")]
    let mut hits = Vec::new();
    loop {
        let scale = (screen_width() / art::W).min(screen_height() / art::H);
        let offset = vec2(
            (screen_width() - art::W * scale) / 2.,
            (screen_height() - art::H * scale) / 2.,
        );
        let (mx, my) = mouse_position();
        let mouse = (vec2(mx, my) - offset) / scale;
        let click = is_mouse_button_pressed(MouseButton::Left);
        #[cfg(feature = "smoke-test")]
        let (mouse, click) = if let Some(s) = &mut smoke {
            s.input(&mut game, &hits)
                .unwrap_or((vec2(-100., -100.), false))
        } else {
            (mouse, click)
        };
        let dt = if smoke_requested {
            0.1
        } else {
            get_frame_time().min(0.1)
        };
        game.update(dt);
        set_camera(&Camera2D {
            render_target: Some(target.clone()),
            ..Camera2D::from_display_rect(Rect::new(0., 0., art::W, art::H))
        });
        let result = ui::draw(&game, mouse, click, get_time() as f32);
        if let Some(event) = result.event {
            game.dispatch(event);
        }
        #[cfg(feature = "smoke-test")]
        {
            hits = result.hits;
        }
        if !smoke_requested && !game.help {
            if is_key_pressed(KeyCode::Escape) {
                game.dispatch(Event::Map);
            }
            if game.screen == Screen::Play {
                if is_key_pressed(KeyCode::Space) {
                    game.dispatch(if game.running {
                        Event::Pause
                    } else {
                        Event::Run
                    });
                }
                if is_key_pressed(KeyCode::Enter) {
                    game.dispatch(Event::Step);
                }
                if is_key_pressed(KeyCode::R) {
                    game.dispatch(Event::Reset);
                }
                let (_, wheel) = mouse_wheel();
                if wheel.abs() > 0. && mouse.x > 846. {
                    game.dispatch(Event::Scroll(if wheel > 0. { -2 } else { 2 }));
                }
            }
        }
        if let Some(event) = game.sound_event.take() {
            audio.play(event, game.progress.volume);
        }
        set_default_camera();
        clear_background(art::INK);
        draw_texture_ex(
            &target.texture,
            offset.x,
            offset.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(art::W * scale, art::H * scale)),
                flip_y: true,
                ..Default::default()
            },
        );
        #[cfg(feature = "smoke-test")]
        if let Some(s) = &mut smoke
            && s.after_frame(&game, &target.texture)
        {
            break;
        }
        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use barkely_highlands::{commands::Action, simulation::Outcome};
    fn game() -> Game {
        let mut g = Game::new(Some(
            std::env::temp_dir().join("barkely-ui-unit-no-save/progress.json"),
        ));
        g.save_allowed = false;
        g.dispatch(Event::Open(0));
        g
    }
    #[test]
    fn step_pause_reset_and_editor() {
        let mut g = game();
        for _ in 0..3 {
            g.dispatch(Event::Add(Action::Forward));
        }
        g.dispatch(Event::Step);
        assert_eq!(g.sim.tick, 1);
        assert!(!g.running);
        g.dispatch(Event::Step);
        assert_eq!(g.sim.tick, 1, "Can't double-step during animation");
        for _ in 0..20 {
            g.update(0.1);
        }
        assert_eq!(g.sim.tick, 1);
        g.dispatch(Event::Run);
        assert_eq!(g.sim.tick, 2);
        g.dispatch(Event::Pause);
        for _ in 0..20 {
            g.update(0.1);
        }
        assert_eq!(g.sim.tick, 2);
        g.dispatch(Event::Step);
        for _ in 0..20 {
            g.update(0.1);
        }
        assert_eq!(g.sim.outcome, Outcome::Success);
        g.dispatch(Event::Reset);
        assert_eq!(g.sim.tick, 0);
        assert_eq!(g.plan.len(), 3);
        g.dispatch(Event::Select(1));
        g.dispatch(Event::Replace);
        g.dispatch(Event::Add(Action::Left));
        assert_eq!(g.plan.len(), 3);
        g.dispatch(Event::Move(-1));
        assert_eq!(g.selected, Some(0));
        g.dispatch(Event::Remove);
        assert_eq!(g.plan.len(), 2);
        g.dispatch(Event::Clear);
        assert!(g.plan.is_empty());
    }
    #[test]
    fn clearing_or_reopening_cancels_replacement() {
        let mut g = game();
        g.dispatch(Event::Add(Action::Forward));
        g.dispatch(Event::Replace);
        g.dispatch(Event::Clear);
        g.dispatch(Event::Add(Action::Left));
        assert_eq!(g.plan, barkely_highlands::mission::cards("L"));
        g.dispatch(Event::Replace);
        g.dispatch(Event::Open(0));
        g.dispatch(Event::Add(Action::Forward));
        assert_eq!(g.plan, barkely_highlands::mission::cards("F"));
    }
    #[test]
    fn locked_missions_cannot_be_opened() {
        let mut g = game();
        g.dispatch(Event::Open(15));
        assert_eq!(g.index, 0);
    }
    #[test]
    fn editing_a_partial_run_rewinds_world() {
        let mut g = game();
        g.dispatch(Event::Add(Action::Forward));
        g.dispatch(Event::Add(Action::Forward));
        g.dispatch(Event::Step);
        for _ in 0..20 {
            g.update(0.1);
        }
        g.dispatch(Event::Add(Action::Left));
        assert_eq!(g.sim.tick, 0);
        assert_eq!(g.cursor, 0);
    }
}
