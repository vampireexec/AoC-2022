#[macro_use]
extern crate lazy_static;
use std::{
    collections::{HashSet, VecDeque},
    env::var,
    fs::read_to_string,
};

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref ADJ: Vec<Delta> = vec![
        vec![-1, 0, 0],
        vec![0, -1, 0],
        vec![0, 0, -1],
        vec![1, 0, 0],
        vec![0, 1, 0],
        vec![0, 0, 1],
    ];
}

type Point = Vec<i32>;
type Delta = Vec<i32>;
type Lava = HashSet<Point>;
type Space = HashSet<Point>;
type Bounds = Vec<Vec<i32>>;

fn unpack(s: &str) -> Point {
    s.split(",")
        .map(|d| i32::from_str_radix(d, 10).unwrap())
        .collect()
}

fn delta(p: &Point, d: &Delta) -> Point {
    vec![p[0] + d[0], p[1] + d[1], p[2] + d[2]]
}

fn get_bounds(lava: &Lava) -> Bounds {
    let xmax = lava.iter().max_by(|a, b| a[0].cmp(&b[0])).unwrap()[0];
    let xmin = lava.iter().min_by(|a, b| a[0].cmp(&b[0])).unwrap()[0];
    let ymax = lava.iter().max_by(|a, b| a[1].cmp(&b[1])).unwrap()[1];
    let ymin = lava.iter().min_by(|a, b| a[1].cmp(&b[1])).unwrap()[1];
    let zmax = lava.iter().max_by(|a, b| a[2].cmp(&b[2])).unwrap()[2];
    let zmin = lava.iter().min_by(|a, b| a[2].cmp(&b[2])).unwrap()[2];
    vec![vec![xmin, xmax], vec![ymin, ymax], vec![zmin, zmax]]
}

fn stretch_bounds(b: Bounds) -> Bounds {
    vec![
        vec![b[0][0] - 1, b[0][1] + 1],
        vec![b[1][0] - 1, b[1][1] + 1],
        vec![b[2][0] - 1, b[2][1] + 1],
    ]
}

fn check_bounds(p: &Point, b: &Bounds) -> bool {
    p[0] >= b[0][0]
        && p[0] <= b[0][1]
        && p[1] >= b[1][0]
        && p[1] <= b[1][1]
        && p[2] >= b[2][0]
        && p[2] <= b[2][1]
}

fn get_space(lava: &Lava) -> Space {
    if lava.len() == 0 {
        return Space::from([vec![0,0,0]]);
    }

    let bounds = stretch_bounds(get_bounds(lava));
    let first = vec![bounds[0][0], bounds[1][0], bounds[2][0]];

    let mut todo = VecDeque::from([first.clone()]);
    let mut visited = Space::from([first.clone()]);
    let mut ret = Space::new();

    while todo.len() > 0 {
        let curr = todo.pop_front().unwrap();

        if lava.contains(&curr) {
            continue;
        } else {
            ret.insert(curr.clone());
        }

        for d in ADJ.iter() {
            let next = delta(&curr, &d);
            if !visited.contains(&next) && check_bounds(&next, &bounds) {
                visited.insert(next.clone());
                todo.push_back(next.clone());
            }
        }
    }
    ret
}

fn main() {
    let lava = Lava::from_iter(INPUT.lines().map(|l| unpack(l)));
    let mut sides = 0;

    for p in &lava {
        for d in ADJ.iter() {
            if !lava.contains(&delta(p, d)) {
                sides += 1;
            }
        }
    }
    println!("{}", sides);

    let space = get_space(&lava);
    sides = 0;
    for p in &lava {
        for d in ADJ.iter() {
            let next = delta(p, d);
            if space.contains(&next) {
                sides += 1;
            }
        }
    }
    println!("{}", sides);
}
