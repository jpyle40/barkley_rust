//! Drives the rendered mouse controls through every mission.
use crate::{game::Game, ui::Hit};
use barkely_highlands::{commands::Card, save::Progress, simulation::Outcome};
use macroquad::prelude::*;
use std::collections::VecDeque;

#[path = "../tests/support/mod.rs"]
mod support;

pub struct Smoke {
    clicks: VecDeque<String>,
    frames: usize,
    started: bool,
}

impl Smoke {
    pub fn new(game: &mut Game) -> Self {
        game.progress = Progress::default();
        game.save_allowed = false;
        Self {
            clicks: VecDeque::from(["help".into(), "close-help".into(), "continue".into()]),
            frames: 0,
            started: false,
        }
    }

    pub fn input(&mut self, game: &mut Game, hits: &[Hit]) -> Option<(Vec2, bool)> {
        self.frames += 1;
        assert!(
            self.frames < 12000,
            "Graphical test timed out on trail {}",
            game.index + 1
        );
        if let Some(id) = self.clicks.front() {
            let hit = hits.iter().find(|h| h.enabled && &h.id == id)?;
            let point = hit.rect.center();
            self.clicks.pop_front();
            return Some((point, true));
        }
        if game.screen != crate::game::Screen::Play {
            return None;
        }
        if !self.started {
            self.clicks.push_back("clear".into());
            if game.plan.is_empty() {
                self.clicks.pop_back();
            }
            for card in support::solve(game.mission()) {
                if let Card::Action(action) = card {
                    self.clicks.push_back(format!("add-{action:?}"));
                }
            }
            self.clicks.push_back("run".into());
            self.started = true;
        } else if game.predicting {
            self.clicks.push_back("skip".into());
        } else if game.sim.outcome == Outcome::Success
            && game.animation >= 1.
            && game.index + 1 < game.missions.len()
        {
            self.clicks.push_back("next".into());
            self.started = false;
        } else {
            assert!(
                !matches!(game.sim.outcome, Outcome::Stopped(_) | Outcome::Finished),
                "Trail {} failed: {}",
                game.index + 1,
                game.sim.feedback
            );
        }
        None
    }

    pub fn after_frame(&mut self, game: &Game, texture: &Texture2D) -> bool {
        if game.sim.outcome == Outcome::Success && game.animation >= 1. {
            let directory = std::env::temp_dir().join("barkely-smoke");
            std::fs::create_dir_all(&directory).expect("create screenshot directory");
            texture.get_texture_data().export_png(
                directory
                    .join(format!("trail-{:02}.png", game.index + 1))
                    .to_str()
                    .unwrap(),
            );
            if game.index + 1 == game.missions.len() {
                assert_eq!(game.progress.completed.len(), game.missions.len());
                println!(
                    "Graphical smoke test passed: all {} missions completed using mouse controls.",
                    game.missions.len()
                );
                return true;
            }
        }
        false
    }
}
