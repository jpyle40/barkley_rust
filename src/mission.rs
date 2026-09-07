use crate::{
    commands::{Action, Card},
    simulation::{Direction, Pos},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Grass,
    Path,
    Tree,
    Water,
    Bridge,
    Fence,
    Rock,
    Bench,
    Gate,
    Switch,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Friend {
    Oliver,
    Henry,
    Betsey,
}
impl Friend {
    pub fn name(self) -> &'static str {
        match self {
            Self::Oliver => "Oliver",
            Self::Henry => "Henry",
            Self::Betsey => "Betsey",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Goal {
    Reach,
    Deliver,
}
#[derive(Clone, Debug)]
pub struct Patrol {
    pub route: Vec<Pos>,
}
impl Patrol {
    pub fn at(&self, tick: usize) -> Pos {
        self.route[tick % self.route.len()]
    }
}

#[derive(Clone, Debug)]
pub struct Mission {
    pub id: &'static str,
    pub title: &'static str,
    pub area: &'static str,
    pub story: &'static str,
    pub tip: &'static str,
    pub success: &'static str,
    pub rows: [&'static str; 7],
    pub start: Pos,
    pub facing: Direction,
    pub friend: Friend,
    pub destination: Pos,
    pub friend_route: Vec<Pos>,
    pub goal: Goal,
    pub patrols: Vec<Patrol>,
    pub item: Option<Pos>,
    pub hidden_item: bool,
    pub prediction: bool,
    pub repeat: bool,
    pub wait: bool,
    pub starter: Vec<Card>,
    pub par: usize,
}
impl Mission {
    pub fn tile(&self, p: Pos) -> Tile {
        if p.x < 0 || p.y < 0 || p.x >= 9 || p.y >= 7 {
            return Tile::Fence;
        }
        match self.rows[p.y as usize].as_bytes()[p.x as usize] {
            b'#' => Tile::Tree,
            b'~' => Tile::Water,
            b'=' => Tile::Bridge,
            b'|' => Tile::Fence,
            b'o' => Tile::Rock,
            b'b' => Tile::Bench,
            b'X' => Tile::Gate,
            b's' => Tile::Switch,
            b':' => Tile::Path,
            _ => Tile::Grass,
        }
    }
    pub fn friend_at(&self, tick: usize) -> Pos {
        if self.friend_route.is_empty() {
            self.destination
        } else {
            self.friend_route[tick % self.friend_route.len()]
        }
    }
    pub fn available(&self) -> Vec<Action> {
        let mut a = vec![Action::Forward, Action::Left, Action::Right];
        if self.wait {
            a.push(Action::Wait);
        }
        if self.item.is_some() {
            a.extend([Action::PickUp, Action::Drop]);
        }
        if self.hidden_item {
            a.push(Action::Sniff);
        }
        a
    }
}

const FIELD: [&str; 7] = [
    "#########",
    "#.......#",
    "#.......#",
    "#:::::::#",
    "#.......#",
    "#.......#",
    "#########",
];
fn base(
    id: &'static str,
    title: &'static str,
    area: &'static str,
    start: (i32, i32),
    end: (i32, i32),
) -> Mission {
    Mission {
        id,
        title,
        area,
        story: "A new trail is waiting. Help Barkely find his friend.",
        tip: "Build a plan, imagine the walk, then watch each command happen.",
        success: "You found the way! Another piece of Highlands is ready to explore.",
        rows: FIELD,
        start: Pos::new(start.0, start.1),
        facing: Direction::East,
        friend: Friend::Oliver,
        destination: Pos::new(end.0, end.1),
        friend_route: vec![],
        goal: Goal::Reach,
        patrols: vec![],
        item: None,
        hidden_item: false,
        prediction: false,
        repeat: false,
        wait: false,
        starter: vec![],
        par: 0,
    }
}
pub fn all() -> Vec<Mission> {
    let mut missions = Vec::new();
    let mut m = base(
        "hello-highlands",
        "A familiar whistle",
        "Park entrance",
        (2, 3),
        (5, 3),
    );
    m.story = "Oliver is waiting just along the path. Barkely knows that whistle! Plan his first little walk.";
    m.tip = "Click Forward three times, then RUN. Each Forward moves one stepping stone.";
    m.success = "There's Oliver! Three small steps, one very happy reunion. Let's explore.";
    m.rows = [
        "#########",
        "#########",
        "##.....##",
        "##:::::##",
        "##.....##",
        "#########",
        "#########",
    ];
    m.par = 3;
    missions.push(m);

    let mut m = base(
        "corner",
        "Around the flower bed",
        "Open field",
        (2, 4),
        (4, 2),
    );
    m.story =
        "Oliver has spotted butterflies near the flowers. Can Barkely turn the corner to join him?";
    m.tip = "A turn changes where Barkely faces. It does not move him. Try STEP to see it happen.";
    m.rows = [
        "#########",
        "#.....#.#",
        "#...::..#",
        "#...:...#",
        "#.:::...#",
        "#.......#",
        "#########",
    ];
    m.par = 5;
    missions.push(m);

    let mut m = base(
        "orientation",
        "Which way is forward?",
        "Picnic lawn",
        (5, 4),
        (2, 2),
    );
    m.facing = Direction::North;
    m.friend = Friend::Betsey;
    m.story = "Betsey has found a shady picnic spot. Barkely starts facing up the park this time.";
    m.tip = "Follow the small arrow at Barkely's paws. Forward always means the way HE is facing.";
    m.rows = [
        "#########",
        "#..b....#",
        "#.::::..#",
        "#....:..#",
        "#....:..#",
        "#b......#",
        "#########",
    ];
    m.par = 6;
    missions.push(m);

    let mut m = base("detour", "The old oak", "Playground edge", (2, 3), (6, 3));
    m.story = "A grand old oak stands in the middle of the trail. Oliver is on the other side.";
    m.tip = "Trees block the way. Go around the oak above or below; both routes can work.";
    m.rows = [
        "#########",
        "#.....b.#",
        "#.......#",
        "#.::#::.#",
        "#.......#",
        "#.o.....#",
        "#########",
    ];
    m.par = 9;
    missions.push(m);

    let mut m = base(
        "debug",
        "A muddled little plan",
        "Playground",
        (2, 4),
        (5, 2),
    );
    m.story = "Henry left a plan for Barkely, but one turn seems muddled. Watch it, then change what you need.";
    m.friend = Friend::Henry;
    m.tip =
        "Try STEP with Henry's plan. Select a card and use Replace to change it. There's no hurry.";
    m.starter = cards("FFFRFF");
    m.par = 6;
    missions.push(m);

    let mut m = base(
        "two-trails",
        "Two ways through",
        "Woodland trail",
        (2, 3),
        (6, 3),
    );
    m.story = "The trail divides around a thicket. Betsey says every safe route is a good route.";
    m.friend = Friend::Betsey;
    m.rows = [
        "#########",
        "#:::::::#",
        "#...#...#",
        "#.::#::.#",
        "#...#...#",
        "#:::::::#",
        "#########",
    ];
    m.tip = "Choose a side of the thicket. After you succeed, you can try a different route.";
    m.par = 11;
    missions.push(m);

    let mut m = base("repeat", "The long meadow", "Meadow", (1, 3), (7, 3));
    m.story = "Oliver races to the far end of the meadow. Barkely's walk has a familiar repeating rhythm.";
    m.repeat = true;
    m.par = 1;
    m.tip = "Add one Forward. Select it, choose 1 card and 6 times, then Group. Six steps fit in one repeat!";
    missions.push(m);

    let mut m = base(
        "pattern",
        "The winding garden",
        "Terrace gardens",
        (1, 5),
        (7, 2),
    );
    m.rows = [
        "#########",
        "#########",
        "######::#",
        "####::::#",
        "##::::###",
        "#:::#####",
        "#########",
    ];
    m.story = "The garden paths climb in little zigzags. Can you spot the same small journey three times?";
    m.repeat = true;
    m.par = 1;
    m.tip = "Try Forward, Forward, Left, Forward, Right. Group those 5 cards and repeat 3 times.";
    missions.push(m);

    let mut m = base(
        "wait",
        "After you, Mudge",
        "Woodland crossing",
        (3, 3),
        (5, 3),
    );
    m.rows = [
        "#########",
        "####:####",
        "####:####",
        "###:::###",
        "####:####",
        "####:####",
        "#########",
    ];
    m.story =
        "Mudge is taking his usual stroll. Barkely can wait politely before crossing his path.";
    m.wait = true;
    m.patrols = vec![Patrol {
        route: positions(&[(4, 2), (4, 3), (4, 4), (4, 3)]),
    }];
    m.tip = "Mudge moves one spot per command, even when you turn. His next spot is marked. Try WAIT first.";
    m.par = 3;
    missions.push(m);

    let mut m = base(
        "patrol",
        "A gap in the patrol",
        "Woodland bend",
        (1, 4),
        (7, 2),
    );
    m.wait = true;
    m.prediction = true;
    m.repeat = true;
    m.rows = [
        "#########",
        "#.......#",
        "#:::::::#",
        "#.#.:.#.#",
        "#:::::::#",
        "#...:...#",
        "#########",
    ];
    m.patrols = vec![Patrol {
        route: positions(&[(4, 2), (4, 3), (4, 4), (4, 5), (4, 4), (4, 3)]),
    }];
    m.story = "The woodland opens into two paths. Study Mudge's patrol and imagine where your plan will end.";
    m.tip = "Before RUN, place a prediction flag. The numbered paw trail shows Mudge's repeating route.";
    m.par = 10;
    missions.push(m);

    let mut m = base(
        "henry",
        "Catch up with Henry",
        "Sports field",
        (2, 3),
        (5, 3),
    );
    m.friend = Friend::Henry;
    m.wait = true;
    m.prediction = true;
    m.friend_route = positions(&[(5, 3), (6, 3), (6, 3), (5, 3)]);
    m.story =
        "Henry is pacing beside the sports field. Plan where he will be when Barkely finishes.";
    m.tip = "Henry's blue trail repeats: right, stay, left, stay. Turns and WAIT also advance his walk.";
    m.par = 3;
    missions.push(m);

    let mut m = base(
        "fetch",
        "One very important ball",
        "Ball lawn",
        (2, 3),
        (2, 3),
    );
    m.item = Some(Pos::new(5, 3));
    m.goal = Goal::Deliver;
    m.story = "Oliver's ball has rolled away. Fetch it and put it down at Oliver's feet.";
    m.tip = "Stand on the ball to Pick up. Turn around with two turns, come back, then Drop.";
    m.par = 10;
    missions.push(m);

    let mut m = base(
        "bridge",
        "Over the silver creek",
        "Creek bridge",
        (2, 4),
        (6, 2),
    );
    m.rows = [
        "####~####",
        "#...~...#",
        "#...~...#",
        "#:::=:::#",
        "#...~...#",
        "#...~...#",
        "####~####",
    ];
    m.friend = Friend::Betsey;
    m.repeat = true;
    m.prediction = true;
    m.story =
        "Betsey is across the creek. The wooden bridge is the only dry way to the other bank.";
    m.tip = "Find the bridge before planning the crossing. Water blocks Barkely; the wooden planks are safe.";
    m.par = 9;
    missions.push(m);

    let mut m = base(
        "gate",
        "The secret garden gate",
        "Hidden garden",
        (2, 4),
        (6, 2),
    );
    m.rows = [
        "#########",
        "#...|...#",
        "#...|...#",
        "#:::X:::#",
        "#.s.|...#",
        "#...|...#",
        "#########",
    ];
    m.friend = Friend::Henry;
    m.story = "Henry found a hidden garden. A golden paw switch opens its gate. Step on it to let Barkely through.";
    m.tip = "Walk onto the golden switch first. The gate stays open for the rest of this attempt.";
    m.par = 12;
    missions.push(m);

    let mut m = base(
        "sniff",
        "A nose for adventure",
        "Wildflower hollow",
        (2, 3),
        (2, 3),
    );
    m.item = Some(Pos::new(6, 3));
    m.hidden_item = true;
    m.goal = Goal::Deliver;
    m.story = "The ball disappeared into the wildflowers. Barkely's nose can help bring it back to Oliver.";
    m.tip = "Sniff gives a direction. Sniff next to the hiding place to reveal the ball, then fetch it.";
    m.par = 13;
    missions.push(m);

    let mut m = base(
        "home",
        "Home before the picnic",
        "Highlands lookout",
        (1, 4),
        (1, 4),
    );
    m.friend = Friend::Betsey;
    m.item = Some(Pos::new(7, 2));
    m.goal = Goal::Deliver;
    m.repeat = true;
    m.wait = true;
    m.prediction = true;
    m.rows = [
        "####~####",
        "#...~...#",
        "#...~...#",
        "#:::=:::#",
        "#...~...#",
        "#...~...#",
        "####~####",
    ];
    m.patrols = vec![Patrol {
        route: positions(&[(6, 2), (6, 3), (6, 4), (6, 3)]),
    }];
    m.story = "One last adventure: fetch the ball across the creek, pass Mudge safely, and return to Betsey's picnic.";
    m.tip = "Break the journey into pieces: reach the bridge, fetch the ball, then find a safe way home.";
    m.success = "Home at last! Barkely found the ball, his friends, and a whole park full of adventures. Highlands is yours to explore again.";
    m.par = 18;
    missions.push(m);
    missions
}
fn positions(points: &[(i32, i32)]) -> Vec<Pos> {
    points.iter().map(|&(x, y)| Pos::new(x, y)).collect()
}
pub fn cards(text: &str) -> Vec<Card> {
    text.chars()
        .map(|c| {
            Card::Action(match c {
                'F' => Action::Forward,
                'L' => Action::Left,
                'R' => Action::Right,
                'W' => Action::Wait,
                'P' => Action::PickUp,
                'D' => Action::Drop,
                'S' => Action::Sniff,
                _ => panic!("Unknown action in authored plan: {c}"),
            })
        })
        .collect()
}
