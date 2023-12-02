#[macro_use]
extern crate lazy_static;
use std::{
    collections::{HashMap, VecDeque},
    env::var,
    fs::read_to_string,
};

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref DEBUG: bool = var("DEBUG").is_ok();
}

fn bounds(barren: &HashMap<Point, i32>) -> (Point, Point) {
    let mut xmin = i32::MAX;
    let mut xmax = i32::MIN;
    let mut ymin = i32::MAX;
    let mut ymax = i32::MIN;
    for (x, y) in barren.keys() {
        if x < &xmin {
            xmin = *x;
        }

        if x > &xmax {
            xmax = *x;
        }

        if y < &ymin {
            ymin = *y;
        }

        if y > &ymax {
            ymax = *y;
        }
    }

    ((xmin, ymin), (xmax, ymax))
}
fn dump(barren: &HashMap<Point, i32>) {
    let (min, max) = bounds(barren);
    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            if barren.contains_key(&(x, y)) {
                print!("🧝");
            } else {
                print!("🟫");
            }
        }
        println!("");
    }
    println!("");
}
type Point = (i32, i32);

#[derive(Debug, Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
}

struct Proposal {
    start: Point,
    end: Point,
    id: i32,
    conflict: bool,
}
fn alone(elf: Point, barren: &HashMap<Point, i32>) -> bool {
    [
        (elf.0 - 1, elf.1 - 1),
        (elf.0, elf.1 - 1),
        (elf.0 + 1, elf.1 - 1),
        (elf.0 - 1, elf.1),
        (elf.0 + 1, elf.1),
        (elf.0 - 1, elf.1 + 1),
        (elf.0, elf.1 + 1),
        (elf.0 + 1, elf.1 + 1),
    ]
    .iter()
    .all(|p| !barren.contains_key(&p))
}

fn consider(elf: Point, dir: Dir, barren: &HashMap<Point, i32>) -> bool {
    match dir {
        Dir::North => [
            (elf.0 - 1, elf.1 - 1),
            (elf.0, elf.1 - 1),
            (elf.0 + 1, elf.1 - 1),
        ]
        .iter()
        .all(|p| !barren.contains_key(p)),
        Dir::South => [
            (elf.0 - 1, elf.1 + 1),
            (elf.0, elf.1 + 1),
            (elf.0 + 1, elf.1 + 1),
        ]
        .iter()
        .all(|p| !barren.contains_key(p)),
        Dir::West => [
            (elf.0 - 1, elf.1 - 1),
            (elf.0 - 1, elf.1),
            (elf.0 - 1, elf.1 + 1),
        ]
        .iter()
        .all(|p| !barren.contains_key(p)),
        Dir::East => [
            (elf.0 + 1, elf.1 - 1),
            (elf.0 + 1, elf.1),
            (elf.0 + 1, elf.1 + 1),
        ]
        .iter()
        .all(|p| !barren.contains_key(p)),
    }
}

fn main() {
    let mut barren = HashMap::new();
    let mut id = 0;
    for (y, line) in INPUT.lines().enumerate() {
        for x in 0..line.len() {
            if &line[x..=x] == "#" {
                barren.insert((x as i32, y as i32), id);
                id += 1;
            }
        }
    }

    if *DEBUG {
        dump(&barren);
    }

    let mut order = VecDeque::from([Dir::North, Dir::South, Dir::West, Dir::East]);
    for round in 1.. {
        let mut proposals: HashMap<Point, Proposal> = HashMap::new();
        for elf in barren.iter() {
            let start = *elf.0;
            if alone(*elf.0, &barren) {
                continue;
            }
            for i in 0..order.len() {
                if consider(start, order[i], &barren) {
                    let end = match order[i] {
                        Dir::North => (start.0, start.1 - 1),
                        Dir::South => (start.0, start.1 + 1),
                        Dir::West => (start.0 - 1, start.1),
                        Dir::East => (start.0 + 1, start.1),
                    };
                    proposals
                        .entry(end)
                        .and_modify(|prop| prop.conflict = true)
                        .or_insert(Proposal {
                            start,
                            end,
                            id: *elf.1,
                            conflict: false,
                        });
                    break;
                }
            }
        }

        let mut no_moves = true;
        for prop in proposals.values() {
            if prop.conflict {
                continue;
            }
            no_moves = false;
            barren.remove(&prop.start);
            barren.insert(prop.end, prop.id);
        }

        if *DEBUG {
            dump(&barren);
        }

        let temp = order.pop_front().unwrap();
        order.push_back(temp);

        if round == 10 {
            let (min, max) = bounds(&barren);
            let mut count = 0;
            for y in min.1..=max.1 {
                for x in min.0..=max.0 {
                    if !barren.contains_key(&(x, y)) {
                        count += 1;
                    }
                }
            }

            println!("part 1 {}", count);
        }

        if no_moves {
            println!("part 2 {}", round);
            break;
        }
    }
}
