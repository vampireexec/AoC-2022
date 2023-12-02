#[macro_use]
extern crate lazy_static;
use std::{collections::HashMap, env::var, fs::read_to_string, sync::mpsc::channel, thread::spawn};

use regex::Regex;

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref RE : Regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    static ref TIME : i32 = i32::from_str_radix(var("TIME").unwrap_or(String::from("24")).as_str(), 10).unwrap();
    static ref PART2 : bool = var("PART2").is_ok();
}

type ResSet = [i32; 4];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Res {
    Ore,
    Clay,
    Glass,
    Crystal,
}
impl From<usize> for Res {
    fn from(res: usize) -> Self {
        match res {
            0 => Res::Ore,
            1 => Res::Clay,
            2 => Res::Glass,
            3 => Res::Crystal,
            _ => panic!("Don't"),
        }
    }
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: i32,
    costs: [ResSet; 4], //each robot, each resource
}

impl From<&str> for Blueprint {
    fn from(input: &str) -> Self {
        let caps = RE
            .captures(input)
            .unwrap()
            .iter()
            .skip(1)
            .map(|m| i32::from_str_radix(m.unwrap().as_str(), 10).unwrap())
            .collect::<Vec<_>>();

        let ore_bot = [caps[1], 0, 0, 0];
        let clay_bot = [caps[2], 0, 0, 0];
        let glass_bot = [caps[3], caps[4], 0, 0];
        let crystal_bot = [caps[5], 0, caps[6], 0];
        Blueprint {
            id: caps[0],
            costs: [ore_bot, clay_bot, glass_bot, crystal_bot],
        }
    }
}

fn credit(amount: ResSet, res: ResSet) -> ResSet {
    [
        res[0] + amount[0],
        res[1] + amount[1],
        res[2] + amount[2],
        res[3] + amount[3],
    ]
}

fn debit(amount: ResSet, res: ResSet) -> ResSet {
    [
        res[0] - amount[0],
        res[1] - amount[1],
        res[2] - amount[2],
        res[3] - amount[3],
    ]
}

fn sufficient(cost: ResSet, res: ResSet) -> bool {
    res[0] >= cost[0] && res[1] >= cost[1] && res[2] >= cost[2] && res[3] >= cost[3]
}

fn build(og: ResSet, robot: Res) -> ResSet {
    match robot {
        Res::Ore => [og[0] + 1, og[1], og[2], og[3]],
        Res::Clay => [og[0], og[1] + 1, og[2], og[3]],
        Res::Glass => [og[0], og[1], og[2] + 1, og[3]],
        Res::Crystal => [og[0], og[1], og[2], og[3] + 1],
    }
}

fn calculate_output(bp: &Blueprint) -> i32 {
    let mut memo = HashMap::new();
    partial_output(bp, [1, 0, 0, 0], [0; 4], *TIME, 0, 0, &mut memo)
}

fn partial_output(
    bp: &Blueprint,
    robos: ResSet,
    res: ResSet,
    t: i32,
    pauses: i32,
    best: i32,
    memo: &mut HashMap<(ResSet, ResSet, i32), i32>,
) -> i32 {
    let memo_key = (robos, res, t);
    if let Some(ret) = memo.get(&memo_key) {
        return *ret;
    }

    let next_t = t - 1;
    if next_t < 0 {
        memo.insert(memo_key, res[Res::Crystal as usize]);
        return res[Res::Crystal as usize];
    }

    if (res[Res::Crystal as usize] + (robos[Res::Crystal as usize] + t) * t) < best {
        return 0;
    }

    let next_res = credit(robos, res);
    let mut ret = best;

    if sufficient(bp.costs[Res::Crystal as usize], res) {
        let next_robos = build(robos, Res::Crystal);

        let debited = debit(bp.costs[Res::Crystal as usize], next_res);
        ret = partial_output(bp, next_robos, debited, next_t, pauses, ret, memo);
    } else {
        let mut built = false;
        // build other robot
        for robo_t in Res::Ore as usize..=Res::Glass as usize {
            if robos[robo_t] < *TIME / 2 && sufficient(bp.costs[robo_t], res) {
                built = true;
                let next_robos = build(robos, robo_t.into());
                let debited = debit(bp.costs[robo_t], next_res);
                let next_ret = partial_output(bp, next_robos, debited, next_t, pauses, ret, memo);
                if next_ret > ret {
                    ret = next_ret
                }
            }
        }

        if built == false || next_t > *TIME / 4 {
            //build nothing
            let next_ret = partial_output(bp, robos, next_res, next_t, pauses + 1, ret, memo);
            if next_ret > ret {
                ret = next_ret
            }
        }
    }

    memo.insert(memo_key, res[Res::Crystal as usize]);
    ret
}

fn main() {
    let blueprints = if *PART2 {
        INPUT
            .lines()
            .map(|l| Blueprint::from(l))
            .take(3)
            .collect::<Vec<_>>()
    } else {
        INPUT
            .lines()
            .map(|l| Blueprint::from(l))
            .collect::<Vec<_>>()
    };

    let mut handlers = vec![];

    let (sender, reciever) = channel::<(i32, i32, i32)>();
    for bp in &blueprints {
        let local_bp = bp.clone();
        let local_sender = sender.clone();
        handlers.push(spawn(move || {
            let output = calculate_output(&local_bp);
            local_sender
                .send((local_bp.id, output, local_bp.id * output))
                .unwrap();
        }));
    }

    let mut results = vec![];
    loop {
        if handlers.iter().all(|h| h.is_finished()) {
            break;
        }

        let r = reciever.recv().unwrap();

        println!("\n{:?}", r);
        results.push(r);
    }

    println!("\n{:?}", results);
    if *PART2 {
        println!("{}", results.iter().fold(1, |acc, r| acc * r.1));
    } else {
        println!("{}", results.iter().map(|r| r.2).sum::<i32>());
    }
}
