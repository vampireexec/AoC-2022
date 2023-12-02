#[macro_use]
extern crate lazy_static;
use std::{env::var, fs::read_to_string};

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref DEBUG: bool = var("DEBUG").is_ok();
}

fn main() {
    let mut dec_sum : i64 = 0;
    for line in INPUT.lines() {
        let digits = line.chars().rev().collect::<Vec<char>>();
        let mut pow : i64 = 1;
        for i in 0..digits.len() {
            let dec_digit = match digits[i] {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => panic!("Bad input")
            };

            dec_sum += pow * dec_digit;
            pow *= 5;
        }
    }
    println!("{}", dec_sum);

    let mut output_decoded = vec![];
    let mut remaining : i64 = dec_sum;
    while remaining > 0 {
        let fives = remaining % 5;
        remaining /= 5;
        output_decoded.push(fives);
    }
    output_decoded.push(0);
    println!("{}", output_decoded.iter().rev().map(|d| d.to_string()).collect::<String>());

    let mut output_encoded = vec![];
    for i in 0..output_decoded.len() {
        match output_decoded[i] {
            -2 => {
                output_encoded.push('=');
            },
            -1 => {
                output_encoded.push('-');
            },
            0 => {
                output_encoded.push('0');
            },
            1 => {
                output_encoded.push('1');
            },
            2 => {
                output_encoded.push('2');
            },
            3 => {
                output_encoded.push('=');
                output_decoded[i+1]+=1;
            },
            4 => {
                output_encoded.push('-');
                output_decoded[i+1]+=1;
            },
            5 => {
                output_encoded.push('0');
                output_decoded[i+1] += 1;
            },
            6 => {
                output_encoded.push('1');
                output_decoded[i+1] += 1;

            }
            _ => panic!("bad digit")
        }
    }
    println!("{}", output_encoded.iter().rev().collect::<String>());
}
