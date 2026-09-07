use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    Forward,
    Left,
    Right,
    Wait,
    PickUp,
    Drop,
    Sniff,
}

impl Action {
    pub fn label(self) -> &'static str {
        match self {
            Self::Forward => "Forward",
            Self::Left => "Turn left",
            Self::Right => "Turn right",
            Self::Wait => "Wait",
            Self::PickUp => "Pick up",
            Self::Drop => "Drop",
            Self::Sniff => "Sniff",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Card {
    Action(Action),
    Repeat { times: u8, body: Vec<Action> },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Instruction {
    pub action: Action,
    pub card: usize,
    pub iteration: u8,
}

pub const MAX_CARDS: usize = 64;
pub const MAX_STEPS: usize = 512;

pub fn compile(cards: &[Card]) -> Result<Vec<Instruction>, &'static str> {
    if cards.len() > MAX_CARDS {
        return Err("This plan is full. Try grouping a pattern.");
    }
    let mut result = Vec::new();
    for (card, item) in cards.iter().enumerate() {
        match item {
            Card::Action(action) => result.push(Instruction {
                action: *action,
                card,
                iteration: 1,
            }),
            Card::Repeat { times, body } => {
                if !(2..=8).contains(times) || body.is_empty() || body.len() > 8 {
                    return Err("A repeat needs 1 to 8 actions, repeated 2 to 8 times.");
                }
                for iteration in 1..=*times {
                    for action in body {
                        result.push(Instruction {
                            action: *action,
                            card,
                            iteration,
                        });
                    }
                }
            }
        }
        if result.len() > MAX_STEPS {
            return Err("That's a very long walk! Try a shorter plan.");
        }
    }
    Ok(result)
}

/// Group a contiguous selection without introducing hidden nested loops.
pub fn group(cards: &mut Vec<Card>, start: usize, count: usize, times: u8) -> bool {
    if count == 0 || count > 8 || start + count > cards.len() || !(2..=8).contains(&times) {
        return false;
    }
    let body: Option<Vec<Action>> = cards[start..start + count]
        .iter()
        .map(|c| match c {
            Card::Action(a) => Some(*a),
            _ => None,
        })
        .collect();
    if let Some(body) = body {
        cards.splice(start..start + count, [Card::Repeat { times, body }]);
        true
    } else {
        false
    }
}

pub fn ungroup(cards: &mut Vec<Card>, index: usize) -> bool {
    if let Some(Card::Repeat { body, .. }) = cards.get(index) {
        let body = body.clone();
        if cards.len() - 1 + body.len() > MAX_CARDS {
            return false;
        }
        cards.splice(index..=index, body.into_iter().map(Card::Action));
        true
    } else {
        false
    }
}
