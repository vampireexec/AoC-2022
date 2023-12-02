use std::{env, fs};

fn input_txt() -> String {
    let path = env::args().nth(1).unwrap_or(String::from("ex1.txt"));
    fs::read_to_string(path).ok().unwrap_or_default()
}

fn main() {
    let input = input_txt();
    part1(&input);
    part2(&input);
}

#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
    Invalid,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Lose,
    Draw,
    Invalid,
}

impl Hand {
    fn shoot(&self, other: &Hand) -> Outcome {
        match (self, other) {
            (Hand::Rock, Hand::Rock) => Outcome::Draw,
            (Hand::Rock, Hand::Paper) => Outcome::Lose,
            (Hand::Rock, Hand::Scissors) => Outcome::Win,
            (Hand::Paper, Hand::Rock) => Outcome::Win,
            (Hand::Paper, Hand::Paper) => Outcome::Draw,
            (Hand::Paper, Hand::Scissors) => Outcome::Lose,
            (Hand::Scissors, Hand::Rock) => Outcome::Lose,
            (Hand::Scissors, Hand::Paper) => Outcome::Win,
            (Hand::Scissors, Hand::Scissors) => Outcome::Draw,
            _ => Outcome::Invalid,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
            Hand::Invalid => 0,
        }
    }
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Invalid => 0,
        }
    }

    fn solve(&self, other: &Hand) -> Hand {
        match (other, self) {
            (Hand::Rock, Outcome::Draw) => Hand::Rock,
            (Hand::Rock, Outcome::Lose) => Hand::Scissors,
            (Hand::Rock, Outcome::Win) => Hand::Paper,
            (Hand::Paper, Outcome::Draw) => Hand::Paper,
            (Hand::Paper, Outcome::Lose) => Hand::Rock,
            (Hand::Paper, Outcome::Win) => Hand::Scissors,
            (Hand::Scissors, Outcome::Draw) => Hand::Scissors,
            (Hand::Scissors, Outcome::Lose) => Hand::Paper,
            (Hand::Scissors, Outcome::Win) => Hand::Rock,
            _ => Hand::Invalid,
        }
    }
}

impl From<&str> for Hand {
    fn from(src: &str) -> Hand {
        match src.as_bytes()[0] {
            b'A' | b'X' => Hand::Rock,
            b'B' | b'Y' => Hand::Paper,
            b'C' | b'Z' => Hand::Scissors,
            _ => Hand::Invalid,
        }
    }
}

impl From<&str> for Outcome {
    fn from(src: &str) -> Outcome {
        match src.as_bytes()[0] {
            b'X' => Outcome::Lose,
            b'Y' => Outcome::Draw,
            b'Z' => Outcome::Win,
            _ => Outcome::Invalid,
        }
    }
}

fn part1(input: &String) {
    println!(
        "{:?}",
        input
            .lines()
            .map(|l| l.split(" ").map(|c| c.into()).collect())
            .map(|p: Vec<Hand>| p[1].shoot(&p[0]).score() + p[1].score())
            .sum::<u32>()
    )
}

fn part2(input: &String) {
    println!(
        "{:?}",
        input
            .lines()
            .map(|l| l.split(" "))
            .map(|mut hs| (
                Hand::from(hs.next().unwrap()),
                Outcome::from(hs.next().unwrap())
            ))
            .map(|p| p.1.solve(&p.0).score() + p.1.score())
            .sum::<u32>()
    )
}
