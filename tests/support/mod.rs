use barkely_highlands::{
    commands::{Action, Card},
    mission::Mission,
    simulation::{Outcome, Simulation},
};
use std::collections::{HashSet, VecDeque};

/// An independent breadth-first search validates authored mission objectives.
/// Only compiled into tests / the explicit smoke-test feature, never normal play.
pub fn solve(m: &Mission) -> Vec<Card> {
    let start = Simulation::new(m);
    let mut queue = VecDeque::from([(start, Vec::<Action>::new())]);
    let mut seen = HashSet::new();
    let period = m
        .patrols
        .iter()
        .map(|p| p.route.len())
        .chain([m.friend_route.len().max(1)])
        .product::<usize>()
        .max(1);
    while let Some((state, path)) = queue.pop_front() {
        if state.goal_met(m) && !path.is_empty() {
            return path.into_iter().map(Card::Action).collect();
        }
        if path.len() >= 100 {
            continue;
        }
        let key = (
            state.pos,
            state.direction,
            state.tick % period,
            state.carrying,
            state.item,
            state.revealed,
            state.gate_open,
        );
        if !seen.insert(key) {
            continue;
        }
        for action in m.available() {
            let mut next = state.clone();
            next.trail.clear();
            next.step(m, action);
            if !matches!(next.outcome, Outcome::Stopped(_)) {
                let mut plan = path.clone();
                plan.push(action);
                queue.push_back((next, plan));
            }
        }
    }
    panic!("Mission {} has no solution", m.id)
}
