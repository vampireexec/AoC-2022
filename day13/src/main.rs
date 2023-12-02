#[macro_use]
extern crate lazy_static;
use std::{cmp::Ordering, env::var, fmt::Display, fs::read_to_string, iter::once};

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Int(u32),
}

impl Packet {
    fn one(x: u32) -> Packet {
        Packet::List(vec![Packet::Int(x)])
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (&Packet::Int(x), &Packet::Int(y)) => x.partial_cmp(&y),
            (&Packet::List(ref x), &Packet::List(ref y)) => {
                let mut r = Ordering::Equal;
                for i in 0..(x.len().max(y.len())) {
                    r = x.get(i).cmp(&y.get(i)); //slick comparison of options
                    if r != Ordering::Equal {
                        break;
                    }
                }
                Some(r)
            }
            (&Packet::Int(x), &Packet::List(_)) => Packet::one(x).partial_cmp(other),
            (&Packet::List(_), &Packet::Int(x)) => self.partial_cmp(&Packet::one(x)),
        }
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl From<&str> for Packet {
    fn from(other: &str) -> Self {
        if other.as_bytes()[0].is_ascii_digit() {
            return Packet::Int(u32::from_str_radix(other, 10).unwrap());
        }
        let mut v = vec![];
        let mut depth = 0;
        let mut last = 1;
        let mut i = 1;
        while i < other.len() - 1 {
            match &other[i..i + 1] {
                "[" => depth += 1,
                "]" => depth -= 1,
                "," => {
                    if depth == 0 {
                        let p = Packet::from(&other[last..i]);
                        v.push(p);
                        last = i + 1; //skip comma
                    }
                }
                _ => (),
            }
            i += 1;
        }
        if depth == 0 && last < i {
            v.push(Packet::from(&other[last..i]));
        }
        Packet::List(v)
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Int(x) => write!(f, "{}", x),
            Packet::List(ref l) => {
                write!(
                    f,
                    "[{}]",
                    l.iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
        }
    }
}

fn main() {
    let lines = (*INPUT).lines().collect::<Vec<&str>>();
    println!(
        "{}",
        lines
            .chunks(3)
            .enumerate()
            .filter(|(_, c)| Packet::from(c[0]) < Packet::from(c[1]))
            .map(|(n, _)| n + 1)
            .sum::<usize>()
    );

    let d1 = Packet::from("[[2]]");
    let d2 = Packet::from("[[6]]");
    
    let mut packets = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| Packet::from(*l))
        .chain(once(d1.clone()))
        .chain(once(d2.clone()))
        .collect::<Vec<Packet>>();
    packets.sort();

    let (n1, _) = packets.iter().enumerate().find(|(_, p)| **p == d1).unwrap();
    let (n2, _) = packets.iter().enumerate().find(|(_, p)| **p == d2).unwrap();
    println!("{}", (n1 + 1) * (n2 + 1));
}
