use std::{env, fs, collections::HashSet};

fn input_txt() -> String {
    let path = env::args().nth(1).unwrap_or(String::from("ex1.txt"));
    fs::read_to_string(path).ok().unwrap_or_default()
}

fn main() {
    let input = input_txt();
    part12(&input, 4);
    part12(&input, 14);
}

fn part12(input: &String, len: usize) {
    for i in (len-1)..input.len() {
        if input[(i-(len-1))..=i].chars().collect::<HashSet<char>>().len() == len {
            println!("{}", i+1);
            break;
        }
    }
}