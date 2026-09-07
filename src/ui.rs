use crate::art::{GOLD, GREEN};
use crate::{
    art::*,
    game::{Event, Game, Screen},
};
use barkely_highlands::{
    commands::{Action, Card},
    simulation::Outcome,
};
use macroquad::prelude::*;

#[cfg(feature = "smoke-test")]
pub struct Hit {
    pub id: String,
    pub rect: Rect,
    pub enabled: bool,
}
pub struct Ui {
    #[cfg(feature = "smoke-test")]
    pub hits: Vec<Hit>,
    pub event: Option<Event>,
    pub mouse: Vec2,
    pub click: bool,
}
impl Ui {
    pub fn new(mouse: Vec2, click: bool) -> Self {
        Self {
            #[cfg(feature = "smoke-test")]
            hits: vec![],
            event: None,
            mouse,
            click,
        }
    }
    fn button(&mut self, id: &str, text: &str, r: Rect, color: Color, enabled: bool, event: Event) {
        let hovered = enabled && r.contains(self.mouse);
        let color = if !enabled {
            color_u8!(225, 227, 210, 255)
        } else if hovered {
            Color::new(
                (color.r + 0.07).min(1.),
                (color.g + 0.07).min(1.),
                (color.b + 0.07).min(1.),
                1.,
            )
        } else {
            color
        };
        round(
            Rect::new(r.x, r.y + 2., r.w, r.h),
            9.,
            color_u8!(32, 56, 43, 30),
        );
        round(r, 9., color);
        let dark = color.r + color.g + color.b > 1.8;
        centered(
            text,
            r,
            if r.h < 30. { 15. } else { 18. },
            if !enabled {
                MUTED
            } else if dark {
                INK
            } else {
                PAPER
            },
        );
        self.hit(id, r, enabled, event);
    }
    fn hit(&mut self, id: &str, r: Rect, enabled: bool, event: Event) {
        if enabled && self.click && r.contains(self.mouse) && self.event.is_none() {
            self.event = Some(event);
        }
        #[cfg(feature = "smoke-test")]
        self.hits.push(Hit {
            id: id.into(),
            rect: r,
            enabled,
        });
        #[cfg(not(feature = "smoke-test"))]
        let _ = id;
    }
}
pub fn draw(g: &Game, mouse: Vec2, click: bool, time: f32) -> Ui {
    background(time);
    let mut ui = Ui::new(mouse, click && !g.help);
    paw(vec2(50., 44.), 1.35, GREEN);
    label("BARKELY'S", 76., 39., 18., GREEN);
    label("HIGHLANDS ADVENTURE", 76., 61., 22., INK);
    let count = g.progress.completed.len();
    label(
        &format!("{count} / 16 trails discovered"),
        870.,
        49.,
        18.,
        MUTED,
    );
    ui.button(
        "volume",
        &format!("Sound {}%", (g.progress.volume * 100.).round() as i32),
        Rect::new(1070., 24., 110., 38.),
        CREAM,
        true,
        Event::Volume,
    );
    ui.button(
        "help",
        "?",
        Rect::new(1192., 24., 42., 38.),
        GREEN,
        true,
        Event::Help,
    );
    match g.screen {
        Screen::Map => map(g, &mut ui, time),
        Screen::Play => play(g, &mut ui, time),
    }
    if let Some(warning) = &g.save_warning {
        round(
            Rect::new(32., H - 29., 1216., 24.),
            6.,
            color_u8!(245, 220, 177, 255),
        );
        let short: String = warning.chars().take(155).collect();
        label(&short, 43., H - 12., 14., INK);
    }
    if g.help {
        #[cfg(feature = "smoke-test")]
        ui.hits.clear();
        ui.event = None;
        ui.click = click;
        draw_rectangle(0., 0., W, H, Color::new(0.08, 0.16, 0.11, 0.62));
        round(Rect::new(280., 142., 720., 610.), 24., PAPER);
        label("A little plan. A big adventure.", 322., 199., 34., INK);
        let paragraphs = [
            (
                "1. PLAN",
                "Click command cards to build a walk. Forward follows Barkely's arrow; a turn changes direction without moving.",
            ),
            (
                "2. PREDICT",
                "Imagine where the plan ends. On prediction trails, click the park to plant a flag before Barkely sets off.",
            ),
            (
                "3. WATCH",
                "RUN plays the whole plan. STEP performs one action. Every action advances Mudge and Henry by one beat, too.",
            ),
            (
                "4. TRY AGAIN",
                "Select a card to move, remove, or replace it. Reset returns everyone to the start and keeps your plan. No lives, no rush.",
            ),
            (
                "PATTERNS",
                "On repeat trails, select the first card, set how many cards and how many times, then Group. Ungroup restores one copy for editing.",
            ),
        ];
        let mut y = 243.;
        for (heading, body) in paragraphs {
            label(heading, 322., y, 17., GREEN);
            y = wrap(body, 322., y + 25., 625., 20., INK) + 24.;
        }
        ui.button(
            "close-help",
            "Let's explore",
            Rect::new(720., 690., 230., 42.),
            GREEN,
            true,
            Event::Help,
        );
    }
    ui
}
fn map(g: &Game, ui: &mut Ui, time: f32) {
    label("Every trail begins with curiosity.", 44., 121., 35., INK);
    label(
        "A park to explore. A friend to find. A plan that's yours.",
        45.,
        153.,
        21.,
        MUTED,
    );
    round(
        Rect::new(40., 185., 344., 655.),
        22.,
        color_u8!(225, 231, 205, 255),
    );
    label("MEET BARKELY", 68., 222., 17., GREEN);
    tree(vec2(105., 376.), 1.7, 4);
    tree(vec2(309., 345.), 1.4, 7);
    draw_ellipse(212., 410., 136., 54., 0., color_u8!(191, 205, 158, 255));
    for i in 0..14 {
        flower(
            vec2(92. + ((i * 43) % 240) as f32, 415. + ((i * 19) % 45) as f32),
            i,
        );
    }
    dog(
        vec2(212., 382.),
        barkely_highlands::simulation::Direction::East,
        2.2,
        false,
        time,
        false,
        false,
    );
    label("Small paws.", 69., 486., 32., INK);
    label("Wonderful possibilities.", 69., 522., 29., INK);
    wrap(
        "Help a curious terrier explore Highlands by planning his next adventure, one command at a time.",
        69.,
        561.,
        285.,
        23.,
        INK,
    );
    label("PLAN  /  PREDICT  /  DISCOVER", 69., 689., 15., GREEN);
    let next = g
        .missions
        .iter()
        .position(|m| !g.progress.completed.contains_key(m.id))
        .unwrap_or(0);
    ui.button(
        "continue",
        if g.progress.completed.is_empty() {
            "Begin the adventure"
        } else {
            "Continue exploring"
        },
        Rect::new(66., 744., 292., 52.),
        GREEN,
        true,
        Event::Open(next),
    );
    round(
        Rect::new(408., 185., 832., 655.),
        24.,
        color_u8!(212, 224, 187, 255),
    );
    for i in 0..18 {
        tree(
            vec2(
                429. + ((i * 173) % 790) as f32,
                226. + ((i * 97) % 575) as f32,
            ),
            0.55,
            i,
        );
    }
    // One continuous trail connects the four chapters in reading order.
    for i in 0..15 {
        let a = node(i) + vec2(84., 47.);
        let b = node(i + 1) + vec2(84., 47.);
        draw_line(a.x, a.y, b.x, b.y, 18., color_u8!(189, 181, 132, 255));
        draw_line(a.x, a.y, b.x, b.y, 12., color_u8!(237, 217, 167, 255));
    }
    for i in 0..16 {
        let p = node(i);
        let m = &g.missions[i];
        let unlocked = g.progress.unlocked(&g.missions, i);
        let complete = g.progress.completed.contains_key(m.id);
        let r = Rect::new(p.x, p.y, 178., 118.);
        let hover = unlocked && r.contains(ui.mouse);
        round(
            Rect::new(r.x, r.y + 4., r.w, r.h),
            14.,
            color_u8!(66, 90, 57, 30),
        );
        round(
            r,
            14.,
            if hover {
                WHITE
            } else if unlocked {
                PAPER
            } else {
                color_u8!(224, 229, 206, 245)
            },
        );
        draw_circle(
            p.x + 23.,
            p.y + 24.,
            14.,
            if complete {
                GREEN
            } else if unlocked {
                GOLD
            } else {
                color_u8!(174, 187, 153, 255)
            },
        );
        centered(
            &(i + 1).to_string(),
            Rect::new(p.x + 9., p.y + 10., 28., 28.),
            17.,
            if complete { PAPER } else { INK },
        );
        label(
            if complete {
                "DISCOVERED"
            } else if unlocked {
                "LET'S EXPLORE"
            } else {
                "UP THE TRAIL"
            },
            p.x + 44.,
            p.y + 28.,
            12.,
            MUTED,
        );
        wrap(
            m.title,
            p.x + 12.,
            p.y + 59.,
            155.,
            19.,
            if unlocked { INK } else { MUTED },
        );
        label(m.area, p.x + 12., p.y + 101., 14., MUTED);
        ui.hit(&format!("mission-{i}"), r, unlocked, Event::Open(i));
    }
    label(
        "Take your time. Every attempt teaches Barkely something.",
        437.,
        822.,
        18.,
        GREEN,
    );
}
fn node(index: usize) -> Vec2 {
    let row = index / 4;
    let col = if row.is_multiple_of(2) {
        index % 4
    } else {
        3 - index % 4
    };
    vec2(430. + col as f32 * 199., 206. + row as f32 * 147.)
}
fn play(g: &Game, ui: &mut Ui, time: f32) {
    let m = g.mission();
    ui.button(
        "map",
        "< Highlands",
        Rect::new(40., 90., 132., 34.),
        CREAM,
        true,
        Event::Map,
    );
    label(
        &format!("TRAIL {:02}  /  {}", g.index + 1, m.area.to_uppercase()),
        189.,
        114.,
        16.,
        GREEN,
    );
    label(m.title, 42., 157., 34., INK);
    wrap(m.story, 43., 181., 766., 18., MUTED);
    board(g, time);
    round(
        Rect::new(40., 773., 771., 94.),
        16.,
        if g.sim.outcome == Outcome::Success {
            color_u8!(220, 235, 204, 255)
        } else {
            CREAM
        },
    );
    let heading = if g.sim.outcome == Outcome::Success {
        "A GOOD DAY FOR AN ADVENTURE"
    } else if matches!(g.sim.outcome, Outcome::Stopped(_) | Outcome::Finished) {
        "LET'S THINK ABOUT THAT WALK"
    } else if g.running {
        "WATCH BARKELY'S PLAN UNFOLD"
    } else {
        "A LITTLE HELP FROM HIGHLANDS"
    };
    label(heading, 58., 798., 14., GREEN);
    wrap(&g.sim.feedback, 58., 824., 730., 18., INK);
    // A compact control strip sits beside the park, outside playable cells.
    ui.button(
        "grid",
        if g.progress.show_grid {
            "Grid on"
        } else {
            "Grid off"
        },
        Rect::new(672., 89., 72., 30.),
        CREAM,
        true,
        Event::Grid,
    );
    ui.button(
        "speed",
        if g.progress.slow { "Slow" } else { "Normal" },
        Rect::new(752., 89., 72., 30.),
        CREAM,
        true,
        Event::Speed,
    );

    round(
        Rect::new(850., 90., 390., 777.),
        20.,
        color_u8!(44, 66, 54, 35),
    );
    round(Rect::new(846., 86., 390., 777.), 20., PAPER);
    label("Barkely's plan", 867., 123., 29., INK);
    let sub = if g.predicting {
        "Plant your prediction flag in the park.".into()
    } else if let Some(active) = g.active {
        format!(
            "Beat {}  /  {}  /  facing {}",
            g.sim.tick,
            active.action.label(),
            g.sim.direction.name()
        )
    } else {
        format!("{} cards  /  click to add, select to edit", g.plan.len())
    };
    label(&sub, 868., 151., 16., MUTED);
    if g.plan.is_empty() {
        round(Rect::new(867., 171., 348., 319.), 12., CREAM);
        paw(vec2(1041., 248.), 2.0, color_u8!(170, 184, 148, 255));
        centered(
            "Where shall we go?",
            Rect::new(883., 286., 317., 40.),
            24.,
            INK,
        );
        wrap(
            "Choose the commands below. Barkely will follow your plan exactly.",
            899.,
            352.,
            274.,
            21.,
            MUTED,
        );
    }
    for (i, card) in g.plan.iter().enumerate().skip(g.scroll).take(8) {
        let r = Rect::new(867., 171. + (i - g.scroll) as f32 * 40., 348., 36.);
        let active = g.active.is_some_and(|a| a.card == i);
        let selected = g.selected == Some(i);
        round(
            r,
            8.,
            if active {
                color_u8!(244, 220, 161, 255)
            } else if selected {
                color_u8!(216, 232, 214, 255)
            } else {
                CREAM
            },
        );
        label(&format!("{:02}", i + 1), r.x + 10., r.y + 24., 16., MUTED);
        let text = match card {
            Card::Action(a) => a.label().into(),
            Card::Repeat { times, body } => {
                let short: Vec<&str> = body
                    .iter()
                    .map(|a| match a {
                        Action::Forward => "F",
                        Action::Left => "L",
                        Action::Right => "R",
                        Action::Wait => "W",
                        Action::PickUp => "P",
                        Action::Drop => "D",
                        Action::Sniff => "S",
                    })
                    .collect();
                format!("Repeat {times}x: {}", short.join(" "))
            }
        };
        label(&text, r.x + 45., r.y + 24., 18., INK);
        if active {
            let suffix = if matches!(card, Card::Repeat { .. }) {
                format!("#{}", g.active.unwrap().iteration)
            } else {
                ">".into()
            };
            label(&suffix, r.x + r.w - 30., r.y + 23., 17., GREEN);
        }
        ui.hit(&format!("card-{i}"), r, g.editing(), Event::Select(i));
    }
    ui.button(
        "scroll-up",
        "Earlier",
        Rect::new(867., 499., 77., 25.),
        CREAM,
        g.scroll > 0,
        Event::Scroll(-8),
    );
    label(
        &format!(
            "{}-{} of {}",
            if g.plan.is_empty() { 0 } else { g.scroll + 1 },
            (g.scroll + 8).min(g.plan.len()),
            g.plan.len()
        ),
        964.,
        517.,
        14.,
        MUTED,
    );
    ui.button(
        "scroll-down",
        "Later",
        Rect::new(1138., 499., 77., 25.),
        CREAM,
        g.scroll + 8 < g.plan.len(),
        Event::Scroll(8),
    );
    let edit = g.editing() && g.selected.is_some();
    ui.button(
        "move-up",
        "Up",
        Rect::new(867., 533., 49., 30.),
        CREAM,
        edit,
        Event::Move(-1),
    );
    ui.button(
        "move-down",
        "Down",
        Rect::new(922., 533., 58., 30.),
        CREAM,
        edit,
        Event::Move(1),
    );
    ui.button(
        "replace",
        if g.replace { "Choose..." } else { "Replace" },
        Rect::new(986., 533., 85., 30.),
        if g.replace { GOLD } else { CREAM },
        edit,
        Event::Replace,
    );
    ui.button(
        "remove",
        "Remove",
        Rect::new(1077., 533., 77., 30.),
        CREAM,
        edit,
        Event::Remove,
    );
    ui.button(
        "clear",
        "Clear",
        Rect::new(1160., 533., 55., 30.),
        CREAM,
        g.editing() && !g.plan.is_empty(),
        Event::Clear,
    );

    if m.repeat {
        label("REPEAT A PATTERN", 868., 587., 12., GREEN);
        ui.button(
            "size-minus",
            "-",
            Rect::new(867., 595., 24., 25.),
            CREAM,
            g.editing(),
            Event::GroupSize(-1),
        );
        label(&format!("{} cards", g.group_size), 897., 613., 15., INK);
        ui.button(
            "size-plus",
            "+",
            Rect::new(948., 595., 24., 25.),
            CREAM,
            g.editing(),
            Event::GroupSize(1),
        );
        ui.button(
            "times-minus",
            "-",
            Rect::new(982., 595., 24., 25.),
            CREAM,
            g.editing(),
            Event::RepeatTimes(-1),
        );
        label(&format!("{}x", g.repeat_times), 1014., 613., 15., INK);
        ui.button(
            "times-plus",
            "+",
            Rect::new(1043., 595., 24., 25.),
            CREAM,
            g.editing(),
            Event::RepeatTimes(1),
        );
        let is_group = g
            .selected
            .and_then(|i| g.plan.get(i))
            .is_some_and(|c| matches!(c, Card::Repeat { .. }));
        ui.button(
            "group",
            if is_group { "Ungroup" } else { "Group" },
            Rect::new(1080., 591., 135., 30.),
            GREEN,
            edit,
            if is_group {
                Event::Ungroup
            } else {
                Event::Group
            },
        );
    } else {
        label(
            if g.replace {
                "Choose a command below to replace this card."
            } else {
                "Select a card above to change your plan."
            },
            868.,
            594.,
            16.,
            MUTED,
        );
    }
    for (i, action) in m.available().iter().enumerate() {
        let r = Rect::new(
            867. + (i % 3) as f32 * 118.,
            632. + (i / 3) as f32 * 39.,
            112.,
            33.,
        );
        ui.button(
            &format!("add-{action:?}"),
            action.label(),
            r,
            if *action == Action::Forward {
                GREEN
            } else {
                CREAM
            },
            g.editing(),
            Event::Add(*action),
        );
    }
    if g.predicting {
        ui.button(
            "skip",
            "Skip prediction",
            Rect::new(867., 755., 348., 43.),
            GOLD,
            true,
            Event::SkipPrediction,
        );
        if let Some(pos) = tile_at(ui.mouse)
            && !g.sim.blocked(m, pos)
        {
            let p = point(pos);
            draw_circle_lines(p.x, p.y, 28., 3., GOLD);
            ui.hit(
                "prediction-spot",
                Rect::new(p.x - CELL / 2., p.y - CELL / 2., CELL, CELL),
                true,
                Event::Predict(pos),
            );
        }
    } else if g.sim.outcome == Outcome::Success {
        let best = g
            .progress
            .completed
            .get(m.id)
            .copied()
            .unwrap_or(g.plan.len());
        label(
            &format!("Best: {best} cards. Optional challenge: {}.", m.par),
            868.,
            744.,
            15.,
            GREEN,
        );
        ui.button(
            "next",
            if g.index == 15 {
                "Highlands explored!"
            } else {
                "On to the next trail >"
            },
            Rect::new(867., 755., 348., 43.),
            GREEN,
            true,
            Event::Next,
        );
    } else {
        ui.button(
            "run",
            if g.running {
                "Pause"
            } else if g.cursor > 0 && matches!(g.sim.outcome, Outcome::Active) {
                "Resume"
            } else {
                "RUN"
            },
            Rect::new(867., 755., 224., 43.),
            GREEN,
            !g.plan.is_empty(),
            if g.running { Event::Pause } else { Event::Run },
        );
        ui.button(
            "step",
            "STEP",
            Rect::new(1100., 755., 115., 43.),
            GOLD,
            !g.running && g.animation >= 1. && !g.plan.is_empty(),
            Event::Step,
        );
    }
    ui.button(
        "reset",
        "Reset / edit plan",
        Rect::new(867., 811., 348., 33.),
        CREAM,
        true,
        Event::Reset,
    );
}
