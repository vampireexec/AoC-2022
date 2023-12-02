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
    let mut tick = 0;
    let mut x = 1;
    let mut sum = 0;
    for token in input.split_whitespace() {
        tick += 1;
        if (x..x + 3).contains(&(tick % 40)) {
            print!("#");
        } else {
            print!(" ");
        }
        if tick % 40 == 0 {
            println!("");
        }
        if tick >= 20 && (tick - 20) % 40 == 0 {
            sum += x * tick;
        }
        if "-0123456789".contains(&token[0..1]) {
            x += i32::from_str_radix(token, 10).unwrap();
        }
    }
    println!("{}", sum);
}