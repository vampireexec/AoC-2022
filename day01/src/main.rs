use std::{env, fs};

fn input_txt() -> String {
    let path = env::args().nth(1).unwrap_or(String::from("ex1.txt"));
    fs::read_to_string(path).ok().unwrap_or_default()
}

fn main() {
    let input = input_txt();
    part12(&input);
}

fn part12(input: &String) {
    let mut elves: Vec<u32> = vec![];
    let mut sum = 0;
    for line in input.lines() {
        if let Ok(x) = u32::from_str_radix(line, 10) {
            sum += x;
        } else {
            elves.push(sum);
            sum = 0;
        }
    }
    elves.sort_by(|a, b| b.cmp(a));
    println!(
        "{}, {}",
        elves.first().unwrap(),
        elves.iter().take(3).sum::<u32>()
    );
}
