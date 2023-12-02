#[macro_use]
extern crate lazy_static;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    env::var,
    fs::read_to_string,
    time::SystemTime,
};

use regex::Regex;

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref PAT: Regex =
        Regex::new(r"Valve (\S\S) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
    static ref ELEPHANT: bool = var("ELEPHANT").is_ok();
    static ref START_TIME: SystemTime = SystemTime::now();
}

#[derive(Debug, Clone, Copy)]
struct Valve {
    _name: u64,
    rate: i32,
    adj: u64,
}

type Cave = HashMap<u64, Valve>;

fn main() {
    let mut lut: HashMap<String, u64> = HashMap::new();
    let mut last = 0u64;
    let mut cave = Cave::new();

    for line in INPUT.lines() {
        let caps = PAT.captures(&line).unwrap();
        let name = caps[1].to_string();
        let rate = i32::from_str_radix(&caps[2], 10).unwrap();
        let adj = caps[3]
            .split(", ")
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();

        if !lut.contains_key(&name) {
            lut.insert(name.clone(), 1 << last);
            last += 1;
        }

        for valve in &adj {
            if !lut.contains_key(valve) {
                lut.insert(valve.clone(), 1 << last);
                last += 1;
            }
        }

        cave.insert(
            lut[&name],
            Valve {
                _name: lut[&name],
                rate,
                adj: (&adj)
                    .iter()
                    .map(|s| lut[s])
                    .reduce(|acc, i| acc | i)
                    .unwrap(),
            },
        );
    }

    let mut travel: HashMap<(u64, u64), i32> = HashMap::new();
    let valve_names = cave.keys().cloned().collect::<Vec<_>>();
    for i in 0..valve_names.len() {
        for j in 0..valve_names.len() {
            let start = valve_names[i];
            let end = valve_names[j];
            if i == j {
                travel.insert((start, start), 0);
                continue;
            }

            let mut que = VecDeque::from([(0, start)]);
            let mut visited = HashSet::<u64>::new();
            while !que.is_empty() {
                let (dist, curr) = que.pop_front().unwrap();
                if curr == end {
                    travel.insert((start, curr), dist);
                    break;
                }
                for i in 0..64 {
                    let adj = 1 << i;
                    if cave[&curr].adj & adj != 0 {
                        if !visited.contains(&adj) {
                            visited.insert(adj);
                            que.push_back((dist + 1, adj));
                        }
                    }
                }
            }
        }
    }

    let start = lut[&String::from("AA")];
    all_possible_paths(&cave, &travel, start);
}

fn all_possible_paths(cave: &Cave, travel: &HashMap<(u64, u64), i32>, start: u64) {
    let valve_set = cave
        .keys()
        .filter(|k| **k != start && cave[*k].rate > 0)
        .cloned()
        .reduce(|acc, k| acc | k)
        .unwrap();

    println!(
        "Max solo {}",
        dfs_solo(cave, travel, start, valve_set, 30, 0)
    );

    println!(
        "Max pair {}",
        search_pair(cave, travel, start, valve_set, 26)
    );
}


fn search_pair(
    cave: &Cave,
    travel: &HashMap<(u64, u64), i32>,
    curr_node: u64,
    remaining: u64,
    time: i32,
) -> i32 {
    let mut max_released = 0;

    let bits = (0..64)
        .map(|i| 1 << i)
        .filter(|b| b & remaining != 0)
        .collect::<Vec<_>>();

    for order in CombinationIter::new(bits.len(), bits.len() / 2) {
        let remaining1 = (&order)
            .iter()
            .map(|i| bits[*i])
            .reduce(|acc, v| acc | v)
            .unwrap();
        
        let release1 = dfs_solo(cave, travel, curr_node, remaining1, time, 0);

        let remaining2 = (&bits)
            .iter()
            .filter(|b| remaining1 & **b == 0)
            .cloned()
            .reduce(|acc, v| acc | v)
            .unwrap();

        let release2 = dfs_solo(cave, travel, curr_node, remaining2, time, 0);

        if release1 + release2 > max_released {
            println!(
                "New max pair {}+{}={} (.{})",
                release1,
                release2,
                release1 + release2,
                START_TIME.elapsed().unwrap().as_millis()
            );
            max_released = release1 + release2;
        }
    }

    max_released
}

fn dfs_solo(
    cave: &Cave,
    travel: &HashMap<(u64, u64), i32>,
    curr_node: u64,
    remaining: u64,
    time: i32,
    best_so_far: i32,
) -> i32 {
    let mut release_ceiling = cave[&curr_node].rate * time;
    for i in 0..64 {
        let valve = 1 << i;
        if valve & remaining != 0 {
            release_ceiling +=
                (cave[&valve].rate * (time - travel[&(curr_node, valve)] - 1)).max(0);
        }
    }

    if release_ceiling < best_so_far {
        return 0;
    }

    let mut max_released = 0;
    for i in 0..64 {
        let next_node = 1 << i;
        if next_node & remaining == 0 {
            continue;
        }

        let next_time = time - travel[&(curr_node, next_node)] - 1;
        if next_time > 0 {
            let mut next_remaining = remaining;
            next_remaining &= !next_node;

            let next_released = dfs_solo(
                cave,
                travel,
                next_node,
                next_remaining,
                next_time,
                max_released,
            );

            if next_released > max_released {
                max_released = next_released;
            }
        }
    }

    let ret = max_released + cave[&curr_node].rate * time;
    ret
}

struct CombinationIter {
    size: usize,
    subset: usize,
    regs: Vec<usize>,
    done: bool,
}

impl CombinationIter {
    fn new(size: usize, subset: usize) -> Self {
        CombinationIter {
            size,
            subset,
            regs: (0..).take(subset).collect(),
            done: false,
        }
    }
}

impl Iterator for CombinationIter {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let ret = self.regs.clone();

        
        while let Some(reg) = self.regs.pop() {
            let next = reg + 1;
            if next <= self.size - (self.subset - self.regs.len()) {
                self.regs.push(next);
                break;
            }
        }

        if self.regs.is_empty() {
            self.done = true;
            return Some(ret);
        }

        while self.regs.len() < self.subset {
            let start = self.regs.last().unwrap() + 1;
            self.regs.push(start);
        }

        Some(ret)
    }
}