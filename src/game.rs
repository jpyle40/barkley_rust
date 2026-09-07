use barkely_highlands::{
    commands::{self, Action, Card, Instruction},
    mission::{self, Mission},
    save::{self, Progress},
    simulation::{Outcome, Pos, Simulation},
};
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Screen {
    Map,
    Play,
}
#[derive(Clone, Copy, Debug)]
pub enum Event {
    Open(usize),
    Map,
    Add(Action),
    Select(usize),
    Remove,
    Move(i32),
    Replace,
    Clear,
    Run,
    Step,
    Reset,
    Pause,
    Scroll(i32),
    Group,
    Ungroup,
    GroupSize(i32),
    RepeatTimes(i32),
    Predict(Pos),
    SkipPrediction,
    Volume,
    Grid,
    Speed,
    Help,
    Next,
}
pub struct Game {
    pub missions: Vec<Mission>,
    pub index: usize,
    pub screen: Screen,
    pub sim: Simulation,
    pub previous: Simulation,
    pub plan: Vec<Card>,
    pub program: Vec<Instruction>,
    pub cursor: usize,
    pub active: Option<Instruction>,
    pub running: bool,
    pub animation: f32,
    pub selected: Option<usize>,
    pub replace: bool,
    pub scroll: usize,
    pub group_size: usize,
    pub repeat_times: u8,
    pub prediction: Option<Pos>,
    pub predicting: bool,
    pub pending_step: bool,
    pub progress: Progress,
    pub save_path: PathBuf,
    pub save_warning: Option<String>,
    pub save_allowed: bool,
    pub help: bool,
    pub sound_event: Option<SoundEvent>,
    pub celebration: f32,
}
#[derive(Clone, Copy)]
pub enum SoundEvent {
    Click,
    Step,
    Success,
    Stop,
}
impl Game {
    pub fn new(path: Option<PathBuf>) -> Self {
        let missions = mission::all();
        let sim = Simulation::new(&missions[0]);
        let save_path = path.unwrap_or_else(save::default_path);
        let (progress, save_warning) = Progress::load(&save_path);
        let save_allowed = save_warning.is_none();
        Self {
            missions,
            index: 0,
            screen: Screen::Map,
            sim: sim.clone(),
            previous: sim,
            plan: vec![],
            program: vec![],
            cursor: 0,
            active: None,
            running: false,
            animation: 1.,
            selected: None,
            replace: false,
            scroll: 0,
            group_size: 1,
            repeat_times: 2,
            prediction: None,
            predicting: false,
            pending_step: false,
            progress,
            save_path,
            save_warning,
            save_allowed,
            help: false,
            sound_event: None,
            celebration: 0.,
        }
    }
    pub fn mission(&self) -> &Mission {
        &self.missions[self.index]
    }
    pub fn editing(&self) -> bool {
        !self.running && self.animation >= 1. && !self.predicting
    }
    pub fn reset(&mut self) {
        self.replace = false;
        self.sim = Simulation::new(self.mission());
        self.previous = self.sim.clone();
        self.program.clear();
        self.cursor = 0;
        self.active = None;
        self.running = false;
        self.animation = 1.;
        self.predicting = false;
        self.prediction = None;
        self.celebration = 0.;
    }
    fn save(&mut self) {
        if self.save_allowed {
            self.save_warning = self
                .progress
                .write(&self.save_path)
                .err()
                .map(|e| format!("Progress is in memory, but couldn't be saved: {e}"));
        }
    }
    fn changed(&mut self) {
        self.reset();
        self.scroll = self.scroll.min(self.plan.len().saturating_sub(8));
    }
    fn launch(&mut self, step: bool, ask: bool) {
        if self.plan.is_empty() {
            self.sim.feedback = "Add a command to Barkely's plan first.".into();
            return;
        }
        if self.animation < 1. {
            return;
        }
        if matches!(
            self.sim.outcome,
            Outcome::Success | Outcome::Finished | Outcome::Stopped(_)
        ) {
            self.reset();
        }
        if self.program.is_empty() {
            match commands::compile(&self.plan) {
                Ok(p) => self.program = p,
                Err(e) => {
                    self.sim.feedback = e.into();
                    return;
                }
            }
        }
        if ask && self.mission().prediction && self.cursor == 0 && self.prediction.is_none() {
            self.predicting = true;
            self.pending_step = step;
            return;
        }
        self.running = !step;
        self.advance();
    }
    fn advance(&mut self) {
        if self.cursor >= self.program.len() {
            return;
        }
        self.previous = self.sim.clone();
        let instruction = self.program[self.cursor];
        self.active = Some(instruction);
        self.scroll = (instruction.card / 8) * 8;
        self.sim
            .step(&self.missions[self.index], instruction.action);
        self.cursor += 1;
        self.animation = 0.;
        self.sound_event = Some(SoundEvent::Step);
    }
    pub fn update(&mut self, dt: f32) {
        self.celebration = (self.celebration - dt).max(0.);
        if self.animation < 1. {
            let duration = if self.progress.slow { 1.25 } else { 0.65 };
            self.animation = (self.animation + dt / duration).min(1.);
            if self.animation >= 1. {
                if matches!(self.sim.outcome, Outcome::Stopped(_)) {
                    self.running = false;
                    self.sound_event = Some(SoundEvent::Stop);
                } else if self.cursor == self.program.len() {
                    self.sim.finish(&self.missions[self.index]);
                    self.running = false;
                    if self.sim.outcome == Outcome::Success {
                        self.progress
                            .complete(self.missions[self.index].id, self.plan.len());
                        self.save();
                        self.sound_event = Some(SoundEvent::Success);
                        self.celebration = 6.;
                    } else {
                        self.sound_event = Some(SoundEvent::Stop);
                    }
                    if let Some(p) = self.prediction {
                        let note = if p == self.sim.pos {
                            " Your prediction matched!"
                        } else {
                            " Your flag and Barkely ended in different places. What changed?"
                        };
                        self.sim.feedback.push_str(note);
                    }
                } else if self.running {
                    self.advance();
                }
            }
        }
    }
    pub fn dispatch(&mut self, event: Event) {
        self.sound_event = Some(SoundEvent::Click);
        match event {
            Event::Open(index) => {
                if self.progress.unlocked(&self.missions, index) {
                    self.index = index;
                    self.screen = Screen::Play;
                    self.plan = self.mission().starter.clone();
                    self.selected = None;
                    self.scroll = 0;
                    self.help = false;
                    self.reset();
                }
            }
            Event::Map => {
                self.running = false;
                self.screen = Screen::Map;
                self.help = false;
            }
            Event::Next => {
                if self.index + 1 < self.missions.len() {
                    self.dispatch(Event::Open(self.index + 1));
                } else {
                    self.screen = Screen::Map;
                }
            }
            Event::Add(action) if self.editing() => {
                if !self.mission().available().contains(&action) {
                    return;
                }
                if self.replace {
                    if let Some(i) = self.selected {
                        self.plan[i] = Card::Action(action);
                    }
                    self.replace = false;
                } else if self.plan.len() < commands::MAX_CARDS {
                    self.plan.push(Card::Action(action));
                    self.selected = Some(self.plan.len() - 1);
                } else {
                    self.sim.feedback =
                        "The plan has 64 cards. Group a pattern or remove a card to make room."
                            .into();
                    return;
                }
                self.changed();
                self.scroll = self.selected.unwrap_or(0).saturating_sub(7);
            }
            Event::Select(i) if self.editing() && i < self.plan.len() => {
                self.selected = Some(i);
            }
            Event::Remove if self.editing() => {
                if let Some(i) = self.selected {
                    self.plan.remove(i);
                    self.selected = if self.plan.is_empty() {
                        None
                    } else {
                        Some(i.min(self.plan.len() - 1))
                    };
                    self.changed();
                }
            }
            Event::Move(delta) if self.editing() => {
                if let Some(i) = self.selected {
                    let to = i as i32 + delta;
                    if to >= 0 && (to as usize) < self.plan.len() {
                        self.plan.swap(i, to as usize);
                        self.selected = Some(to as usize);
                        self.changed();
                    }
                }
            }
            Event::Replace if self.editing() => {
                self.replace = self.selected.is_some() && !self.replace;
            }
            Event::Clear if self.editing() => {
                self.plan.clear();
                self.selected = None;
                self.changed();
            }
            Event::Reset => {
                self.reset();
            }
            Event::Run if !self.predicting => self.launch(false, true),
            Event::Step if !self.predicting => self.launch(true, true),
            Event::Pause => {
                self.running = false;
            }
            Event::Scroll(delta) => {
                self.scroll = (self.scroll as i32 + delta)
                    .clamp(0, self.plan.len().saturating_sub(8) as i32)
                    as usize;
            }
            Event::Group if self.editing() && self.mission().repeat => {
                if let Some(i) = self.selected {
                    if commands::group(&mut self.plan, i, self.group_size, self.repeat_times) {
                        self.changed();
                    } else {
                        self.sim.feedback = "Select the first card of a pattern. Group 1 to 8 ordinary cards starting there.".into();
                    }
                } else {
                    self.sim.feedback = "Select the first card you want to repeat.".into();
                }
            }
            Event::Ungroup if self.editing() => {
                if let Some(i) = self.selected
                    && commands::ungroup(&mut self.plan, i)
                {
                    self.changed();
                }
            }
            Event::GroupSize(d) => {
                self.group_size = (self.group_size as i32 + d).clamp(1, 8) as usize
            }
            Event::RepeatTimes(d) => {
                self.repeat_times = (self.repeat_times as i32 + d).clamp(2, 8) as u8
            }
            Event::Predict(p) if self.predicting => {
                self.prediction = Some(p);
                self.predicting = false;
                self.launch(self.pending_step, false);
            }
            Event::SkipPrediction if self.predicting => {
                self.predicting = false;
                self.launch(self.pending_step, false);
            }
            Event::Volume => {
                self.progress.volume = if self.progress.volume >= 0.99 {
                    0.
                } else {
                    (self.progress.volume + 0.25).min(1.)
                };
                self.save();
            }
            Event::Grid => {
                self.progress.show_grid = !self.progress.show_grid;
                self.save();
            }
            Event::Speed => {
                self.progress.slow = !self.progress.slow;
                self.save();
            }
            Event::Help => {
                self.help = !self.help;
                self.running = false;
            }
            _ => {}
        }
    }
}
