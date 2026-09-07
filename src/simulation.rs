use crate::{
    commands::Action,
    mission::{Goal, Mission, Tile},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}
impl Pos {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn distance(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    pub fn right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
    pub fn left(self) -> Self {
        self.right().right().right()
    }
    pub fn ahead(self, p: Pos) -> Pos {
        let (x, y) = match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        };
        Pos::new(p.x + x, p.y + y)
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::North => "up",
            Self::East => "right",
            Self::South => "down",
            Self::West => "left",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    Ready,
    Active,
    Success,
    Stopped(String),
    Finished,
}

#[derive(Clone, Debug)]
pub struct Simulation {
    pub pos: Pos,
    pub direction: Direction,
    pub tick: usize,
    pub carrying: bool,
    pub item: Option<Pos>,
    pub revealed: bool,
    pub gate_open: bool,
    pub outcome: Outcome,
    pub feedback: String,
    pub trail: Vec<Pos>,
}
impl Simulation {
    pub fn new(m: &Mission) -> Self {
        Self {
            pos: m.start,
            direction: m.facing,
            tick: 0,
            carrying: false,
            item: m.item,
            revealed: !m.hidden_item,
            gate_open: false,
            outcome: Outcome::Ready,
            feedback: m.tip.into(),
            trail: vec![m.start],
        }
    }
    pub fn blocked(&self, m: &Mission, p: Pos) -> bool {
        match m.tile(p) {
            Tile::Tree | Tile::Water | Tile::Fence | Tile::Rock | Tile::Bench => true,
            Tile::Gate => !self.gate_open,
            _ => false,
        }
    }
    pub fn goal_met(&self, m: &Mission) -> bool {
        match m.goal {
            Goal::Reach => self.pos == m.friend_at(self.tick),
            Goal::Deliver => !self.carrying && self.item == Some(m.friend_at(self.tick)),
        }
    }
    /// Every primitive action advances the world by one beat, including turns and WAIT.
    /// Actors move simultaneously; sharing a destination or swapping places is unsafe.
    pub fn step(&mut self, m: &Mission, action: Action) {
        if matches!(
            self.outcome,
            Outcome::Stopped(_) | Outcome::Success | Outcome::Finished
        ) {
            return;
        }
        self.outcome = Outcome::Active;
        let old = self.pos;
        let mut target = old;
        let mut stop = None;
        self.feedback = match action {
            Action::Forward => {
                target = self.direction.ahead(old);
                if self.blocked(m, target) {
                    stop = Some(
                        match m.tile(target) {
                            Tile::Water => {
                                "Splash? Better find a bridge. Barkely stops at the creek."
                            }
                            Tile::Gate => "The gate is closed. The golden paw switch opens it.",
                            _ => "Barkely found a blocked path. Which turn could help?",
                        }
                        .to_string(),
                    );
                    target = old;
                }
                "One paw-step forward.".into()
            }
            Action::Left => {
                self.direction = self.direction.left();
                "Barkely turns left, staying in the same spot.".into()
            }
            Action::Right => {
                self.direction = self.direction.right();
                "Barkely turns right, staying in the same spot.".into()
            }
            Action::Wait => "Barkely waits for one beat. The rest of the park keeps moving.".into(),
            Action::PickUp => {
                if self.item == Some(old) && !self.carrying && self.revealed {
                    self.item = None;
                    self.carrying = true;
                    "Got it! Now bring the ball back.".into()
                } else {
                    stop = Some("No ball to pick up here. Find it first, then try again.".into());
                    String::new()
                }
            }
            Action::Drop => {
                if self.carrying {
                    self.item = Some(old);
                    self.carrying = false;
                    "Barkely gently puts the ball down.".into()
                } else {
                    stop = Some("Barkely isn't carrying a ball yet.".into());
                    String::new()
                }
            }
            Action::Sniff => {
                if let Some(item) = self.item {
                    if old.distance(item) <= 1 {
                        self.revealed = true;
                        "Woof! The ball is right here. Its hiding place is revealed!".into()
                    } else {
                        let horizontal = if item.x > old.x { "right" } else { "left" };
                        let vertical = if item.y > old.y { "down" } else { "up" };
                        let toward = if (item.x - old.x).abs() >= (item.y - old.y).abs() {
                            horizontal
                        } else {
                            vertical
                        };
                        format!("Sniff sniff... the ball's scent leads {toward} on the map.")
                    }
                } else {
                    "Sniff sniff... fresh grass and a very happy dog.".into()
                }
            }
        };
        for actor in &m.patrols {
            let before = actor.at(self.tick);
            let after = actor.at(self.tick + 1);
            if target == after || (target == before && old == after) {
                stop = Some(
                    "Mudge got there first. Watch his pattern, then try waiting somewhere safe."
                        .into(),
                );
                target = old;
            }
        }
        self.tick += 1;
        self.pos = target;
        self.trail.push(target);
        if m.tile(target) == Tile::Switch {
            self.gate_open = true;
            self.feedback = "Click! The paw switch opened the garden gate.".into();
        }
        if let Some(reason) = stop {
            self.feedback = reason.clone();
            self.outcome = Outcome::Stopped(reason);
        }
    }
    /// Goals are checked at the end, so walking past a friend does not silently fix a plan.
    pub fn finish(&mut self, m: &Mission) {
        if matches!(self.outcome, Outcome::Stopped(_)) {
            return;
        }
        if self.goal_met(m) {
            self.outcome = Outcome::Success;
            self.feedback = m.success.into();
        } else {
            self.outcome = Outcome::Finished;
            self.feedback = "That's the end of the plan. Compare where Barkely stopped with your goal, then edit and try again.".into();
        }
    }
}
