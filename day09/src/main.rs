use std::{collections::HashSet, env, fs};

fn input_txt() -> String {
    let path = env::args().nth(1).unwrap_or(String::from("ex1.txt"));
    fs::read_to_string(path).ok().unwrap_or_default()
}

fn main() {
    let input = input_txt();
    part1(&input);
    part2(&input);
}

fn _print_visits(visited: &HashSet<Point>) {
    let xmax = visited.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let xmin = visited.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let ymax = visited.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let ymin = visited.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    for y in (ymin..ymax).rev() {
        for x in xmin..=xmax {
            if visited.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
fn _print_rope(rope: &Vec<Point>) {
    for y in (-30..30).rev() {
        for x in -30..30 {
            if let Some(n) = rope.iter().enumerate().find(|p| *p.1 == (x, y)) {
                print!("{}", n.0);
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
type Point = (i32, i32);
type UpdateTailError = (Point, Point, Point);
type UpdateTailResult = Result<Point, UpdateTailError>;

fn update_tail(head: Point, tail: Point) -> UpdateTailResult {
    match (head.0 - tail.0, head.1 - tail.1) {
        (-1, -1) | (0, -1) | (1, -1) | (-1, 0) | (0, 0) | (1, 0) | (-1, 1) | (0, 1) | (1, 1) => {
            Ok(tail)
        }

        (-2, -1) => Ok((tail.0 - 1, tail.1 - 1)),
        (-2, 0) => Ok((tail.0 - 1, tail.1)),
        (-2, 1) => Ok((tail.0 - 1, tail.1 + 1)),

        (2, -1) => Ok((tail.0 + 1, tail.1 - 1)),
        (2, 0) => Ok((tail.0 + 1, tail.1)),
        (2, 1) => Ok((tail.0 + 1, tail.1 + 1)),

        (-1, -2) => Ok((tail.0 - 1, tail.1 - 1)),
        (0, -2) => Ok((tail.0, tail.1 - 1)),
        (1, -2) => Ok((tail.0 + 1, tail.1 - 1)),

        (-1, 2) => Ok((tail.0 - 1, tail.1 + 1)),
        (0, 2) => Ok((tail.0, tail.1 + 1)),
        (1, 2) => Ok((tail.0 + 1, tail.1 + 1)),

        (-2, -2) => Ok((tail.0 - 1, tail.1 - 1)),
        (-2, 2) => Ok((tail.0 - 1, tail.1 + 1)),
        (2, -2) => Ok((tail.0 + 1, tail.1 - 1)),
        (2, 2) => Ok((tail.0 + 1, tail.1 + 1)),

        _ => Err((head, tail, (head.0 - tail.0, head.1 - tail.1))),
    }
}

type UpdateHeadError = ();
type UpdateHeadResult = Result<Point, UpdateHeadError>;
fn update_head(cmd: &str, head: Point) -> UpdateHeadResult {
    match cmd {
        "U" => Ok((head.0, head.1 + 1)),
        "D" => Ok((head.0, head.1 - 1)),
        "L" => Ok((head.0 - 1, head.1)),
        "R" => Ok((head.0 + 1, head.1)),
        _ => Err(()),
    }
}

fn part1(input: &String) {
    let mut head: Point = (0, 0);
    let mut tail: Point = (0, 0);
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(tail);

    for line in input.lines() {
        let args: Vec<&str> = line.split_ascii_whitespace().collect();
        let ds = i32::from_str_radix(args[1], 10).unwrap();
        for _ in 0..ds {
            head = update_head(args[0], head).unwrap();
            tail = update_tail(head, tail).unwrap();
            visited.insert(tail);
        }
    }

    println!("{}", visited.len());
}

fn part2(input: &String) {
    let mut rope: Vec<Point> = vec![(0, 0); 10];
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert((0, 0));

    for line in input.lines() {
        let args: Vec<&str> = line.split_ascii_whitespace().collect();
        let ds = i32::from_str_radix(args[1], 10).unwrap();
        for _ in 0..ds {
            rope[0] = update_head(args[0], rope[0]).unwrap();
            let mut head = rope[0];
            for i in 1..rope.len() {
                rope[i] = update_tail(head, rope[i]).unwrap();
                head = rope[i];
            }
            visited.insert(*rope.last().unwrap());
        }
    }

    println!("{}", visited.len());
}
