#[macro_use]
extern crate lazy_static;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    env::var,
    fs::read_to_string,
};

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref DEBUG: bool = var("DEBUG").is_ok();
}

type Point = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Facing {
    fn from(c: char) -> Self {
        match c {
            '^' => Facing::Up,
            'v' => Facing::Down,
            '<' => Facing::Left,
            '>' => Facing::Right,
            _ => panic!("Unknown facing"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Blizzard {
    face: Facing,
    p: Point,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Mount {
    bliz: Vec<Blizzard>,
    start: Point,
    end: Point,
    max: Point,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    step: i32,
    curr: Point,
}

impl Mount {
    fn inc_bliz(&self, b: Blizzard, t: i32) -> Point {
        match b.face {
            Facing::Up => (b.p.0, (b.p.1 - t).rem_euclid(self.max.1 + 1)),
            Facing::Down => (b.p.0, (b.p.1 + t).rem_euclid(self.max.1 + 1)),
            Facing::Left => ((b.p.0 - t).rem_euclid(self.max.0 + 1), b.p.1),
            Facing::Right => ((b.p.0 + t).rem_euclid(self.max.0 + 1), b.p.1),
        }
    }

    fn step(&self, t: i32) -> Vec<Blizzard> {
        self.bliz
            .iter()
            .map(|b| Blizzard {
                p: self.inc_bliz(*b, t),
                face: b.face,
            })
            .collect()
    }

    fn adj(&self, state: State) -> Vec<Point> {
        let p0 = state.curr;
        [(0, -1), (0, 1), (-1, 0), (1, 0), (0, 0)]
            .iter()
            .filter_map(|ds| {
                let p1 = (p0.0 + ds.0, p0.1 + ds.1);
                if p1 == self.start
                    || p1 == self.end
                    || (p1.0 >= 0 && p1.0 <= self.max.0 && p1.1 >= 0 && p1.1 <= self.max.1)
                {
                    Some(p1)
                } else {
                    None
                }
            })
            .collect()
    }

    fn dump(&self, state: State) {
        println!("{:?} / {:?} => {:?}", state.curr, self.max, self.end);
        print!(" 🟦");
        for x in 0..=self.max.0 {
            if x == self.start.0 {
                if state.curr == self.start {
                    print!("🧝");
                } else {
                    print!("⬜");
                }
            } else {
                print!("🟦");
            }
        }
        println!("🟦");
        for y in 0..=self.max.1 {
            print!(" 🟦");
            'grid: for x in 0..=self.max.0 {
                for b in &self.step(state.step) {
                    if (x, y) == b.p {
                        match b.face {
                            Facing::Up => print!("⏫"),
                            Facing::Down => print!("⏬"),
                            Facing::Left => print!("⏪"),
                            Facing::Right => print!("⏩"),
                        }
                        continue 'grid;
                    }
                }
                if state.curr == (x, y) {
                    print!("🧝");
                } else {
                    print!("⬜");
                }
            }

            println!("🟦");
        }
        print!(" 🟦");
        for x in 0..=self.max.0 {
            if x == self.end.0 {
                print!("⬜");
            } else {
                print!("🟦");
            }
        }
        println!("🟦");
    }
}

fn load() -> Mount {
    let mut mount: Mount = Mount {
        bliz: vec![],
        start: (-1, -1),
        end: (-1, -1),
        max: (-1, -1),
    };
    let mut last_open = -1;

    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => continue,
                '.' => last_open = x as i32 - 1,
                '^' | 'v' | '<' | '>' => mount.bliz.push(Blizzard {
                    face: Facing::from(c),
                    p: (x as i32 - 1, y as i32 - 1),
                }),
                _ => panic!("Unknown character in input {}", c),
            }

            if mount.max.0 < x as i32 - 1 {
                mount.max.0 = x as i32 - 1;
            }

            if mount.max.1 < y as i32 - 2 {
                mount.max.1 = y as i32 - 2;
            }
        }
        if y as i32 - 1 == -1 {
            mount.start = (last_open, -1);
        }
    }
    mount.end = (last_open, mount.max.1 + 1);
    mount
}

fn main() {
    let mount = load();

    if *DEBUG {
        mount.dump(State {
            step: 0,
            curr: mount.start,
        });
    }
    let mut state = round(
        &mount,
        State {
            step: 0,
            curr: mount.start,
        },
        mount.end,
    );
    println!("{:?}", state);
    state = round(&mount, state, mount.start);
    println!("{:?}", state);
    state = round(&mount, state, mount.end);
    println!("{:?}", state);
}

fn round(initial: &Mount, initial_state: State, goal: Point) -> State {
    let mut queue = VecDeque::from([initial_state]);
    let mut visited = HashSet::new();
    let mut blizzards = HashMap::from([(0, initial.bliz.clone())]);

    while let Some(state) = queue.pop_front() {
        if *DEBUG {
            initial.dump(state);
        }

        let opts = initial.adj(state);
        'check_opt: for opt in opts {
            let next = State {
                curr: opt,
                step: state.step + 1,
            };
            if opt == goal {
                return next;
            }

            let bvec = blizzards
                .entry(next.step)
                .or_insert(initial.step(next.step));
            for b in bvec {
                if opt == b.p {
                    continue 'check_opt;
                }
            }

            if !visited.contains(&next) {
                visited.insert(next);
                queue.push_back(next);
            }
        }
    }

    panic!("Did not find path");
}
