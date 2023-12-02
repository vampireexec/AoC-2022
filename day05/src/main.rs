use std::{collections::VecDeque, env, fs};

fn input_txt() -> String {
    let path = env::args().nth(1).unwrap_or(String::from("ex1.txt"));
    fs::read_to_string(path).ok().unwrap_or_default()
}

fn main() {
    let input = input_txt();
    part1(&input);
    part2(&input);
}

fn parse_stacks(input: &String) -> (usize, Vec<VecDeque<char>>) {
    let mut stacks: Vec<VecDeque<char>> =
        Vec::from_iter(std::iter::repeat(VecDeque::new()).take(9).clone());
    let mut skip = 0;
    for line in input.lines().take_while(|l| !l.starts_with(" 1")) {
        line.as_bytes()
            .chunks(4)
            .map(|ch| ch[1] as char)
            .enumerate()
            .filter(|p| p.1.is_ascii_alphabetic())
            .for_each(|(i, c)| stacks[i].push_front(c));
        skip += 1;
    }
    skip += 2;
    (skip, stacks)
}

fn part1(input: &String) {
    let lines: Vec<&str> = input.lines().collect();
    let (skip, mut stacks) = parse_stacks(input);
    for line in &lines[skip..] {
        let indexes: Vec<usize> = line
            .chars()
            .filter(|c| c.is_numeric() || c.is_whitespace())
            .collect::<String>()
            .split_whitespace()
            .map(|s| usize::from_str_radix(s, 10).unwrap())
            .collect();
        for _ in 0..indexes[0] {
            let temp = stacks[indexes[1] - 1].pop_back().unwrap();
            stacks[indexes[2] - 1].push_back(temp);
        }
    }

    println!(
        "{}",
        stacks.iter().map(|s| s.back().unwrap()).collect::<String>()
    );
}

fn part2(input: &String) {
    let lines: Vec<&str> = input.lines().collect();
    let (skip, mut stacks) = parse_stacks(input);

    for line in &lines[skip..] {
        let indexes: Vec<usize> = line
            .chars()
            .filter(|c| c.is_numeric() || c.is_whitespace())
            .collect::<String>()
            .split_whitespace()
            .map(|s| usize::from_str_radix(s, 10).unwrap())
            .collect();
        let mut temp = vec![];
        for _ in 0..indexes[0] {
            temp.push(stacks[indexes[1] - 1].pop_back().unwrap());
        }
        temp.reverse();
        for c in temp {
            stacks[indexes[2] - 1].push_back(c);
        }
    }

    println!(
        "{}",
        stacks.iter().map(|s| s.back().unwrap()).collect::<String>()
    );
}
