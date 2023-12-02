use std::{
    collections::{HashMap, HashSet},
    env, fs,
};
#[macro_use]
extern crate lazy_static;

fn input_txt() -> String {
    let path = env::args().nth(1).unwrap_or(String::from("ex1.txt"));
    fs::read_to_string(path).ok().unwrap_or_default()
}

fn main() {
    let input = input_txt();
    part1(&input);
    part2(&input);
}

lazy_static! {
    static ref SCORES: HashMap<u8, u32> = (b'a'..=b'z').chain(b'A'..=b'Z').zip(1..).collect();
}

fn priority(l: &str) -> Option<&u32> {
    let s: HashSet<u8> = l[..l.len() / 2].bytes().collect();
    (*SCORES).get(
        &l[l.len() / 2..]
            .bytes()
            .filter(|x| s.contains(x))
            .next()
            .unwrap(),
    )
}

fn part1(inp: &String) {
    println!("{}", inp.lines().map(|l| priority(l).unwrap()).sum::<u32>());
}

fn part2(input: &String) {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut score = 0;
    for group in lines.chunks(3) {
        score += *SCORES
            .get(
                group
                    .iter()
                    .map(|g| g.bytes().collect::<HashSet<u8>>())
                    .reduce(|acc, x| acc.intersection(&x).cloned().collect())
                    .unwrap()
                    .iter()
                    .next()
                    .unwrap(),
            )
            .unwrap();
    }
    println!("{}", score);
}
