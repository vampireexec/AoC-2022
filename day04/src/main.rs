use std::{env, fs, ops::RangeInclusive};

fn input_txt() -> String {
    let path = env::args().nth(1).unwrap_or(String::from("ex1.txt"));
    fs::read_to_string(path).ok().unwrap_or_default()
}

fn main() {
    let input = input_txt();
    println!("{}", part12(&input, fully_contains));
    println!("{}", part12(&input, overlaps));
}

fn parse(i: &str) -> u32 {
    u32::from_str_radix(i, 10).unwrap()
}

fn fully_contains(r: &(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    (r.0.contains(r.1.start()) && r.0.contains(r.1.end()))
    || (r.1.contains(r.0.start()) && r.1.contains(r.0.end()))
}

fn overlaps(r: &(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    r.0.contains(r.1.start())
    || r.0.contains(r.1.end())
    || r.1.contains(r.0.start())
    || r.1.contains(r.0.end())
}

fn part12(input: &String, check: fn(&(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool) -> usize {
    input
        .lines()
        .map(|l| l.split(&[',', '-']).map(parse).collect::<Vec<u32>>())
        .map(|p| (p[0]..=p[1], p[2]..=p[3]))
        .filter(check)
        .count()
}
