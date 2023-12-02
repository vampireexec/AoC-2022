#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    env::var,
    fs::read_to_string,
};

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref RE: Regex = Regex::new(r"([a-z]+): (?:(\d+)|([a-z]+) ([+-/*]) ([a-z]+))").unwrap();
    static ref TRACE: bool = var("TRACE").is_ok();
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        match s {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            _ => panic!("unknown operations"),
        }
    }
}

#[derive(Debug, Clone)]
enum Exp {
    Literal(String, i64),
    Equation(String, String, Op, String),
}

impl Exp {
    fn name(&self) -> &String {
        match self {
            Exp::Literal(name, _) => &name,
            Exp::Equation(name, _, _, _) => &name,
        }
    }

    fn eval(&self, lut: &HashMap<String, Exp>) -> i64 {
        match self {
            Exp::Literal(name, x) => {
                if *TRACE {
                    println!("{} = {}", name, x)
                }
                *x
            }
            Exp::Equation(name, a, op, b) => {
                let ka = lut[a].eval(lut);
                let kb = lut[b].eval(lut);
                if *TRACE {
                    println!("{} = {} ({}) {:?} {} ({})", name, a, ka, op, b, kb);
                }
                match op {
                    Op::Add => ka + kb,
                    Op::Sub => ka - kb,
                    Op::Mul => ka * kb,
                    Op::Div => ka / kb
                }
            }
        }
    }
}
fn main() {
    let mut lut = HashMap::new();
    let mut rev: HashMap<_, String> = HashMap::new();

    for line in INPUT.lines() {
        let caps = RE.captures(line).unwrap();
        let exp = if caps.get(2).is_some() {
            Exp::Literal(
                caps[1].to_string(),
                i64::from_str_radix(&caps[2], 10).unwrap(),
            )
        } else {
            let name = &caps[1];
            let a = &caps[3];
            let op = &caps[4];
            let b = &caps[5];
            rev.insert(a.to_string(), name.to_string());
            rev.insert(b.to_string(), name.to_string());
            Exp::Equation(name.to_string(), a.to_string(), Op::from(op), b.to_string())
        };

        lut.insert(exp.name().clone(), exp);
    }

    println!("* Part 1 {}", lut["root"].eval(&lut));
    println!("");

    //figure out which half of root needs to be solved for
    let mut curr = "humn";
    let mut human_half = HashSet::from(["humn".to_string()]);
    loop {
        curr = &rev[curr];
        human_half.insert(curr.to_string());

        if rev[curr] == "root" {
            break;
        }
    }

    let mut human_lut = HashMap::from([("root".to_string(), Exp::Literal("root".to_string(), 0))]);
    let mut c;
    if let Exp::Equation(_, a, _, b) = &lut["root"] {
        if human_half.contains(a) {
            human_lut.insert(
                b.to_string(),
                Exp::Literal(b.to_string(), lut[b].eval(&lut)),
            );
            human_lut.insert(
                a.to_string(),
                Exp::Equation(a.to_string(), b.to_string(), Op::Add, "root".to_string()),
            );
            c = a;
        } else {
            human_lut.insert(
                a.to_string(),
                Exp::Literal(a.to_string(), lut[a].eval(&lut)),
            );
            human_lut.insert(
                b.to_string(),
                Exp::Equation(b.to_string(), a.to_string(), Op::Add, "root".to_string()),
            );
            c = b;
        }
    } else {
        panic!("Root must be an equation.");
    }

    loop {
        if let Exp::Equation(_, a, op, b) = &lut[c] {
            if human_half.contains(a) {
                let k = lut[b].eval(&lut);
                let inverse = match op {
                    Op::Add => Op::Sub,
                    Op::Sub => Op::Add,
                    Op::Mul => Op::Div,
                    Op::Div => Op::Mul,
                };
                human_lut.insert(b.to_string(), Exp::Literal(b.to_string(), k));
                human_lut.insert(
                    a.to_string(),
                    Exp::Equation(a.to_string(), c.to_string(), inverse, b.to_string()),
                );
                c = a;
            } else {
                let k = lut[a].eval(&lut);
                human_lut.insert(a.to_string(), Exp::Literal(a.to_string(), k));
                let exp = match op {
                    Op::Add => Exp::Equation(b.to_string(), c.to_string(), Op::Sub, a.to_string()),
                    Op::Mul => Exp::Equation(b.to_string(), c.to_string(), Op::Div, a.to_string()),
                    Op::Sub => Exp::Equation(b.to_string(), a.to_string(), Op::Sub, c.to_string()),
                    Op::Div => Exp::Equation(b.to_string(), a.to_string(), Op::Div, c.to_string())
                };
                human_lut.insert(b.to_string(), exp);
                c = b;
            }
        } else {
            panic!("Should only see equations on this path")
        }

        if c == "humn" {
            break;
        }
    }

    println!("* Part 2 {}", human_lut["humn"].eval(&human_lut));
}
