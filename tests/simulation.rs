mod support;
use barkely_highlands::{
    commands::{self, Action, Card},
    mission::{self, Patrol},
    save::Progress,
    simulation::{Direction, Outcome, Pos, Simulation},
};

fn play(index: usize, plan: &[Card]) -> Simulation {
    let m = &mission::all()[index];
    let mut s = Simulation::new(m);
    for i in commands::compile(plan).unwrap() {
        s.step(m, i.action);
    }
    s.finish(m);
    s
}
#[test]
fn every_mission_has_a_real_solution() {
    for (index, m) in mission::all().iter().enumerate() {
        assert!(m.rows.iter().all(|r| r.len() == 9));
        let plan = support::solve(m);
        let s = play(index, &plan);
        assert_eq!(s.outcome, Outcome::Success, "{}", m.title);
        println!("{}: {} actions", m.title, plan.len());
        for patrol in &m.patrols {
            for i in 0..patrol.route.len() {
                assert!(!Simulation::new(m).blocked(m, patrol.at(i)));
                assert!(patrol.at(i).distance(patrol.at(i + 1)) <= 1);
            }
        }
    }
}
#[test]
fn turns_change_orientation_without_moving() {
    let m = &mission::all()[0];
    let mut s = Simulation::new(m);
    s.step(m, Action::Left);
    assert_eq!(s.pos, m.start);
    assert_eq!(s.direction, Direction::North);
    s.step(m, Action::Right);
    assert_eq!(s.direction, Direction::East);
    assert_eq!(s.tick, 2);
    for _ in 0..4 {
        s.step(m, Action::Right);
    }
    assert_eq!(s.direction, Direction::East);
}
#[test]
fn forward_is_relative_and_boundaries_stop() {
    let m = &mission::all()[0];
    let mut s = Simulation::new(m);
    s.step(m, Action::Left);
    s.step(m, Action::Forward);
    assert_eq!(s.pos, Pos::new(2, 2));
    s.step(m, Action::Forward);
    assert_eq!(s.pos, Pos::new(2, 2));
    assert!(matches!(s.outcome, Outcome::Stopped(_)));
    let tick = s.tick;
    s.step(m, Action::Right);
    assert_eq!(s.tick, tick);
}
#[test]
fn goal_is_checked_at_end_not_when_walking_past() {
    assert_eq!(play(0, &mission::cards("FFF")).outcome, Outcome::Success);
    assert_eq!(play(0, &mission::cards("FFFF")).outcome, Outcome::Finished);
}
#[test]
fn multiple_solutions_are_accepted() {
    assert_eq!(
        play(0, &mission::cards("RLLRFFF")).outcome,
        Outcome::Success
    );
    assert_eq!(play(0, &mission::cards("FFF")).outcome, Outcome::Success);
    assert_eq!(
        play(5, &mission::cards("LFFRFFFFRFF")).outcome,
        Outcome::Success
    );
    assert_eq!(
        play(5, &mission::cards("RFFLFFFFLFF")).outcome,
        Outcome::Success
    );
}
#[test]
fn wait_is_required_at_the_first_crossing() {
    assert!(matches!(
        play(8, &mission::cards("FF")).outcome,
        Outcome::Stopped(_)
    ));
    assert_eq!(play(8, &mission::cards("WFF")).outcome, Outcome::Success);
}
#[test]
fn patrol_collision_detects_swapping_positions() {
    let mut m = mission::all()[0].clone();
    m.patrols = vec![Patrol {
        route: vec![Pos::new(3, 3), Pos::new(2, 3)],
    }];
    let mut s = Simulation::new(&m);
    s.step(&m, Action::Forward);
    assert!(matches!(s.outcome, Outcome::Stopped(_)));
    assert_eq!(s.pos, m.start);
}
#[test]
fn turning_also_advances_patrols() {
    let m = &mission::all()[8];
    let mut s = Simulation::new(m);
    s.step(m, Action::Left);
    assert_eq!(m.patrols[0].at(s.tick), Pos::new(4, 3));
    assert_eq!(s.pos, m.start);
}
#[test]
fn fetching_requires_pickup_and_delivery() {
    assert_eq!(
        play(
            11,
            &mission::cards("FFFP RR FFF D".replace(' ', "").as_str())
        )
        .outcome,
        Outcome::Success
    );
    assert_ne!(
        play(11, &mission::cards("FFFRRFFF")).outcome,
        Outcome::Success
    );
    assert!(matches!(
        play(11, &mission::cards("P")).outcome,
        Outcome::Stopped(_)
    ));
    assert!(matches!(
        play(11, &mission::cards("D")).outcome,
        Outcome::Stopped(_)
    ));
}
#[test]
fn hidden_ball_needs_a_nearby_sniff() {
    assert!(matches!(
        play(14, &mission::cards("FFFFP")).outcome,
        Outcome::Stopped(_)
    ));
    assert_eq!(
        play(14, &mission::cards("FFFFSPRRFFFFD")).outcome,
        Outcome::Success
    );
    let m = &mission::all()[14];
    let mut s = Simulation::new(m);
    s.step(m, Action::Sniff);
    assert!(!s.revealed);
    assert!(s.feedback.contains("right"));
}
#[test]
fn switch_opens_gate_and_reset_closes_it() {
    let m = &mission::all()[13];
    let mut s = Simulation::new(m);
    assert!(s.blocked(m, Pos::new(4, 3)));
    s.step(m, Action::Wait);
    assert!(s.gate_open);
    assert!(!s.blocked(m, Pos::new(4, 3)));
    assert!(!Simulation::new(m).gate_open);
}
#[test]
fn repeated_patterns_have_correct_source_highlights() {
    let mut plan = mission::cards("FFLFR");
    assert!(commands::group(&mut plan, 0, 5, 3));
    let code = commands::compile(&plan).unwrap();
    assert_eq!(code.len(), 15);
    assert_eq!(code[5].iteration, 2);
    assert_eq!(code[14].card, 0);
    assert_eq!(play(7, &plan).outcome, Outcome::Success);
    assert!(commands::ungroup(&mut plan, 0));
    assert_eq!(plan, mission::cards("FFLFR"));
}
#[test]
fn malformed_and_excessive_programs_are_rejected() {
    assert!(
        commands::compile(&[Card::Repeat {
            times: 0,
            body: vec![Action::Forward]
        }])
        .is_err()
    );
    assert!(
        commands::compile(&[Card::Repeat {
            times: 2,
            body: vec![]
        }])
        .is_err()
    );
    assert!(commands::compile(&vec![Card::Action(Action::Forward); 65]).is_err());
    assert!(
        commands::compile(&vec![
            Card::Repeat {
                times: 8,
                body: vec![Action::Forward; 8]
            };
            9
        ])
        .is_err()
    );
}
#[test]
fn debug_plan_is_actually_wrong() {
    let m = &mission::all()[4];
    assert_ne!(play(4, &m.starter).outcome, Outcome::Success);
    assert_eq!(play(4, &mission::cards("FFFLFF")).outcome, Outcome::Success);
}
#[test]
fn moving_friend_is_evaluated_at_current_time() {
    assert_eq!(play(10, &mission::cards("FFF")).outcome, Outcome::Success);
    assert_ne!(
        play(10, &mission::cards("FFFW W".replace(' ', "").as_str())).outcome,
        Outcome::Success
    );
}
#[test]
fn save_round_trip_progression_and_best_score() {
    let dir = std::env::temp_dir().join(format!("barkely-test-{}", std::process::id()));
    let path = dir.join("progress.json");
    let mut p = Progress::default();
    let missions = mission::all();
    assert!(!p.unlocked(&missions, 1));
    p.complete(missions[0].id, 7);
    p.complete(missions[0].id, 3);
    p.complete(missions[0].id, 9);
    p.volume = 0.;
    p.show_grid = true;
    p.write(&path).unwrap();
    let (loaded, warning) = Progress::load(&path);
    assert!(warning.is_none());
    assert_eq!(loaded.completed[missions[0].id], 3);
    assert!(loaded.unlocked(&missions, 1));
    assert!(!loaded.unlocked(&missions, 2));
    assert!(loaded.show_grid);
    assert_eq!(loaded.volume, 0.);
    std::fs::write(&path, b"damaged save").unwrap();
    let (_, warning) = Progress::load(&path);
    assert!(warning.is_some());
    assert_eq!(std::fs::read(&path).unwrap(), b"damaged save");
    std::fs::remove_dir_all(dir).unwrap();
}
#[test]
fn deterministic_replay() {
    let plan = support::solve(&mission::all()[15]);
    let a = play(15, &plan);
    let b = play(15, &plan);
    assert_eq!(a.trail, b.trail);
    assert_eq!(a.tick, b.tick);
    assert_eq!(a.item, b.item);
    assert_eq!(a.outcome, b.outcome);
}
