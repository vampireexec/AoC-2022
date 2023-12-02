#[macro_use]
extern crate lazy_static;
use std::{collections::HashMap, env::var, fs::read_to_string, sync::Mutex};

use regex::Regex;

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref RE: Regex = Regex::new(r"\d+|[RL]").unwrap();
    static ref DEBUG: bool = var("DEBUG").is_ok();
    static ref TRACE: bool = var("TRACE").is_ok();
    static ref TRACE_PATH: bool = var("TRACE_PATH").is_ok();
    static ref PART2: bool = var("PART2").is_ok();
    static ref START: String = var("START").unwrap_or_default();
    static ref INSTR: String = var("INSTR").unwrap_or_default();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Facing {
    fn reverse(&self) -> Self {
        match self {
            Facing::Right => Facing::Left,
            Facing::Down => Facing::Up,
            Facing::Left => Facing::Right,
            Facing::Up => Facing::Down,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Move(i32),
    Left,
    Right,
}

impl From<&str> for Dir {
    fn from(s: &str) -> Self {
        match s {
            "R" => Dir::Right,
            "L" => Dir::Left,
            _ => Dir::Move(i32::from_str_radix(&s, 10).unwrap()),
        }
    }
}

type Point = (i32, i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cretin {
    pos: Point,
    face: Facing,
}

struct Tiles {
    tiles: HashMap<Point, Tile>,
    max: Point,
}

impl Tiles {
    fn new(mut tiles: HashMap<Point, Tile>) -> Self {
        let max = (
            *tiles.iter().map(|((x, _), _)| x).max().unwrap() as i32,
            *tiles.iter().map(|((_, y), _)| y).max().unwrap() as i32,
        );
        for x in -1..=(max.0 + 1) {
            for y in -1..=(max.1 + 1) {
                if !tiles.contains_key(&(x, y)) {
                    tiles.insert((x, y), Tile::Null);
                }
            }
        }

        Tiles { tiles, max }
    }

    fn wrap_steps(&self, c1: Cretin) -> Cretin {
        let mut c2 = c1.step();
        'null_skip: loop {
            if !*PART2 {
                c2.pos.0 = c2.pos.0.rem_euclid(self.max.0 + 1);
                c2.pos.1 = c2.pos.1.rem_euclid(self.max.1 + 1);
            }
            match self.tiles[&c2.pos] {
                Tile::Wall => {
                    wall_trace(c2);
                    return c1;
                }
                Tile::Open => return c2,
                Tile::Null => {
                    if *PART2 {
                        let uncn = c2.unstep();
                        for warp in &*WARP_INPUT {
                            if let Some(wpcn) = warp.warp_if_contained(uncn) {
                                warp_trace(warp.label, uncn, wpcn);
                                c2 = wpcn;
                                continue 'null_skip;
                            }
                        }
                        panic!("Did not find warp.");
                    } else {
                        c2 = c2.step();
                    }
                }
            }
        }
    }
}
lazy_static! {
    static ref TRACE_MAP: Mutex<HashMap<Point, Cretin>> = Mutex::new(HashMap::new());
}

fn trace(c: Cretin) {
    if *TRACE_PATH {
        TRACE_MAP.lock().unwrap().insert(c.pos, c);
    }

    if *TRACE {
        println!("{:?}", c);
    }
}

fn warp_trace(c: char, from: Cretin, to: Cretin) {
    if *TRACE {
        println!("Warp {} from {:?} to {:?}", c, from, to);
    }
}

fn wall_trace(cn: Cretin) {
    if *TRACE {
        println!("Hit wall at {:?}", cn);
    }
}

impl Cretin {
    fn step(&self) -> Self {
        let mut cn = *self;
        match cn.face {
            Facing::Right => cn.pos.0 += 1,
            Facing::Down => cn.pos.1 += 1,
            Facing::Left => cn.pos.0 -= 1,
            Facing::Up => cn.pos.1 -= 1,
        }
        cn
    }

    fn unstep(&self) -> Self {
        let mut cn = *self;
        match cn.face {
            Facing::Right => cn.pos.0 -= 1,
            Facing::Down => cn.pos.1 -= 1,
            Facing::Left => cn.pos.0 += 1,
            Facing::Up => cn.pos.1 += 1,
        }
        cn
    }

    fn apply(&self, dir: Dir, map: &Tiles) -> Cretin {
        let mut ret = *self;
        match (dir, self.face) {
            (Dir::Move(ds), Facing::Up)
            | (Dir::Move(ds), Facing::Down)
            | (Dir::Move(ds), Facing::Left)
            | (Dir::Move(ds), Facing::Right) => {
                for _ in 0..ds {
                    ret = map.wrap_steps(ret);
                    trace(ret);
                }
            }

            (Dir::Left, Facing::Up) => ret.face = Facing::Left,
            (Dir::Left, Facing::Down) => ret.face = Facing::Right,
            (Dir::Left, Facing::Left) => ret.face = Facing::Down,
            (Dir::Left, Facing::Right) => ret.face = Facing::Up,

            (Dir::Right, Facing::Up) => ret.face = Facing::Right,
            (Dir::Right, Facing::Down) => ret.face = Facing::Left,
            (Dir::Right, Facing::Left) => ret.face = Facing::Up,
            (Dir::Right, Facing::Right) => ret.face = Facing::Down,
        };
        trace(ret);
        ret
    }
}

fn main() {
    let lines = INPUT.lines().collect::<Vec<_>>();
    let mut tiles = HashMap::new();
    let mut dirs = vec![];
    let mut y = 0;
    while lines[y] != "" {
        for x in 0..lines[y].len() {
            let s = lines[y].get(x..=x).unwrap();
            let p: Point = (x as i32, y as i32);

            match s {
                " " => tiles.insert(p, Tile::Null),
                "." => tiles.insert(p, Tile::Open),
                "#" => tiles.insert(p, Tile::Wall),
                _ => panic!("bad input ({}, {})", x, y),
            };
        }
        y += 1
    }

    y += 1;
    if INSTR.is_empty() {
        for dir in RE.find_iter(lines[y]) {
            dirs.push(Dir::from(dir.as_str()));
        }
    } else {
        for dir in RE.find_iter(&*INSTR) {
            dirs.push(Dir::from(dir.as_str()));
        }
    }

    let map = Tiles::new(tiles);
    let mut cretin = Cretin {
        pos: (0, 0),
        face: Facing::Right,
    };
    for x in 0..=map.max.0 {
        if map.tiles[&(x, 0)] == Tile::Open {
            cretin.pos = (x, 0);
            break;
        }
    }

    if !START.is_empty() {
        let ps = START.split(",").collect::<Vec<&str>>();
        cretin.pos.0 = i32::from_str_radix(ps[0], 10).unwrap();
        cretin.pos.1 = i32::from_str_radix(ps[1], 10).unwrap();
    }

    trace(cretin);

    if *DEBUG {
        dump(&map, &cretin);
    }

    for dir in dirs {
        cretin = cretin.apply(dir, &map);
        if *DEBUG {
            println!("{:?}", dir);
            dump(&map, &cretin);
        }
    }
    if *TRACE_PATH {
        dump_trace(&map, &TRACE_MAP.lock().unwrap());
    }

    println!(
        "{}x1000 + {}x4 + {} = {}",
        cretin.pos.1 + 1,
        cretin.pos.0 + 1,
        cretin.face as i32,
        (cretin.pos.1 + 1) * 1000 + (cretin.pos.0 + 1) * 4 + cretin.face as i32
    );
}

fn dump(map: &Tiles, cretin: &Cretin) {
    dump_trace(map, &HashMap::from([(cretin.pos, cretin.clone())]));
}

fn dump_trace(map: &Tiles, cretins: &HashMap<Point, Cretin>) {
    for y in 0..=map.max.1 {
        for x in 0..=map.max.0 {
            let p = (x, y);

            if cretins.contains_key(&p) {
                let cretin = cretins[&p];
                match cretin.face {
                    Facing::Up => print!("🔼"),
                    Facing::Down => print!("🔽"),
                    Facing::Left => print!("⏪"),
                    Facing::Right => print!("⏩"),
                }
            } else if map.tiles.contains_key(&p) {
                match map.tiles[&p] {
                    Tile::Open => print!("🟫"),
                    Tile::Wall => print!("🟥"),
                    Tile::Null => print!("⬛"),
                }
            } else {
                print!("💢")
            }
        }
        println!("");
    }
    println!("");
}

lazy_static! {
    static ref WARP_INPUT: Vec<Warp> = {
        let e1 = Edge::new((0, 150), Facing::Left, 50);
        let e2 = Edge::new((49, 150), Facing::Right, 50);
        let e3 = Edge::new((0, 199), Facing::Down, 50);
        let e4 = Edge::new((50, 149), Facing::Down, 50);
        let e5 = Edge::new((99, 100), Facing::Right, 50);
        let e6 = Edge::new((99, 50), Facing::Right, 50);
        let e7 = Edge::new((50, 50), Facing::Left, 50);
        let e8 = Edge::new((0, 100), Facing::Up, 50);
        let e9 = Edge::new((0, 100), Facing::Left, 50);
        let ea = Edge::new((50, 0), Facing::Left, 50);
        let eb = Edge::new((100, 49), Facing::Down, 50);
        let ec = Edge::new((149, 0), Facing::Right, 50);
        let ed = Edge::new((50, 0), Facing::Up, 50);
        let eh = Edge::new((100, 0), Facing::Up, 50);

        vec![
            Warp::new(e1, ed, '①'),
            Warp::new(e2, e4, '②'),
            Warp::new(e3, eh, '③'),
            Warp::new(e4, e2, '④'),
            Warp::new(e5, ec, '⑤'),
            Warp::new(e6, eb, '⑥'),
            Warp::new(e7, e8, '⑦'),
            Warp::new(e8, e7, '⑧'),
            Warp::new(e9, ea, '⑨'),
            Warp::new(ea, e9, '🅰'),
            Warp::new(eb, e6, '🅱'),
            Warp::new(ec, e5, '🅲'),
            Warp::new(ed, e1, '🅳'),
            Warp::new(eh, e3, '🅷'),
        ]
    };
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    origin: Point,
    face: Facing, // out from center
    size: i32,
}

impl Edge {
    fn new(origin: Point, face: Facing, size: i32) -> Self {
        Edge { origin, face, size }
    }
}

#[derive(Debug, Clone, Copy)]
struct Warp {
    enter: Edge,
    exit: Edge,
    label: char,
}

impl Warp {
    fn new(enter: Edge, exit: Edge, label: char) -> Self {
        assert_eq!(enter.size, exit.size);
        Warp { enter, exit, label }
    }

    fn warp_if_contained(&self, cn: Cretin) -> Option<Cretin> {
        let face = self.enter.face;
        let origin = self.enter.origin;
        let size = self.enter.size;

        if cn.face != face {
            return None;
        }

        match cn.face {
            Facing::Right | Facing::Left => {
                if !(origin.1..(origin.1 + size)).contains(&cn.pos.1) || !(origin.0 == cn.pos.0) {
                    return None;
                }
            }
            Facing::Up | Facing::Down => {
                if !(origin.0..(origin.0 + size)).contains(&cn.pos.0) || !(origin.1 == cn.pos.1) {
                    return None;
                }
            }
        }

        let ds = match self.enter.face {
            Facing::Right | Facing::Left => cn.pos.1 % self.enter.size,
            Facing::Up | Facing::Down => cn.pos.0 % self.enter.size,
        };

        let pos = match (self.enter.face, self.exit.face) {
            (Facing::Right, Facing::Left)
            | (Facing::Left, Facing::Right)
            | (Facing::Up, Facing::Left)
            | (Facing::Down, Facing::Right) => (self.exit.origin.0, self.exit.origin.1 + ds),

            (Facing::Up, Facing::Down)
            | (Facing::Down, Facing::Up)
            | (Facing::Right, Facing::Down)
            | (Facing::Left, Facing::Up) => (self.exit.origin.0 + ds, self.exit.origin.1),

            (Facing::Right, Facing::Right)
            | (Facing::Left, Facing::Left)
            | (Facing::Up, Facing::Right)
            | (Facing::Down, Facing::Left) => (
                self.exit.origin.0,
                self.exit.origin.1 + self.exit.size - ds - 1,
            ),

            (Facing::Right, Facing::Up)
            | (Facing::Left, Facing::Down)
            | (Facing::Down, Facing::Down)
            | (Facing::Up, Facing::Up) => (
                self.exit.origin.0 + self.exit.size - ds - 1,
                self.exit.origin.1,
            ),
        };

        let face = self.exit.face.reverse();
        Some(Cretin { pos, face })
    }
}
