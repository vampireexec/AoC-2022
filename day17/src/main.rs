#[macro_use]
extern crate lazy_static;
use std::{
    collections::{HashSet, VecDeque},
    env::var,
    fs::read_to_string,
    iter::repeat,
};

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref DUMP : bool = var("DUMP").is_ok();
    static ref CYCLE : bool = var("CYCLE").is_ok();
    static ref BLOCKS: Vec<Block> = vec![
        Block::from([(0,0), (1,0), (2, 0), (3,0)]), // -
        Block::from([(1,0), (0,1), (1,1), (2,1), (1,2)]), // +
        Block::from([(0,0), (1,0), (2,0), (2,1), (2,2)]), // L backwards
        Block::from([(0,0), (0,1), (0,2), (0,3)]), // |
        Block::from([(0,0), (1,0), (0,1), (1,1)]) // []
    ];
}

type Point = (i64, i64);
type Block = HashSet<Point>;

fn offset_block(b: &Block, xoff: i64, yoff: i64) -> Block {
    b.iter().map(|(x, y)| (x + xoff, y + yoff)).collect()
}

fn check_wall(b: &Block, fill: &Block) -> bool {
    b.iter()
        .filter(|p| p.0 < 0 || p.0 > 6 || fill.contains(&p))
        .next()
        .is_some()
}

fn check_below(b: &Block, fill: &Block) -> bool {
    b.iter().filter(|p| fill.contains(p)).next().is_some()
}

fn get_topo(fill: &Block) -> Block {
    if fill.len() == 0 {
        return Block::new();
    }

    let yedge = fill.iter().map(|p| p.1).max().unwrap() + 1;
    let mut todo = VecDeque::from([(0, yedge)]);
    let mut visited = Block::from([(0, yedge)]);
    let mut ret = Block::new();

    while todo.len() > 0 {
        let curr = todo.pop_front().unwrap();
        if fill.contains(&curr) {
            ret.insert(curr);
            continue;
        }

        for dx in -1..=1 {
            for dy in -1..=1 {
                let next = (curr.0 + dx, curr.1 + dy);
                if !visited.contains(&next) && next.1 <= yedge && next.0 >= 0 && next.0 <= 6 {
                    visited.insert(next);
                    todo.push_back(next);
                }
            }
        }
    }
    ret
}

fn truncate(b: &Block) -> Block { //set lowest point's Y = 0
    let ymin = b.iter().map(|p| p.1).min().unwrap();
    offset_block(b, 0, -ymin)
}

fn main() {
    let mut jet_i = 0;
    let mut block_i = 0;
    let mut other = Block::from_iter((0..=6).zip(repeat(-1)));
    let mut curr: Block;

    other = get_topo(&other);

    let block_count = i64::from_str_radix(&var("BLOCKS").unwrap(), 10).unwrap();
    let mut t = 0;
    let mut cycle: Vec<((Block, usize, usize), i64)> = vec![];
    let mut bc = 1;

    while bc <= block_count {
        let ymax = *(&other.iter().map(|p| p.1.clone()).max().unwrap());

        if *CYCLE {
            //Running version where we look for a cycle

            //key is Top surface + index to jets + block to be dropped
            let key = (truncate(&other), jet_i, block_i);
            let needle = cycle
                .iter()
                .enumerate()
                .filter(|(_, (k, _))| k == &key)
                .next();

            if needle.is_none() {
                cycle.push((key, ymax)); //ymax before curr block drops
            } else {
                let (idx, (_, start_max)) = needle.unwrap();

                println!("Cycle reached {} {} {} {}", idx, key.1, key.2, start_max);
                dump(&Block::new(), &key.0);

                let dy = ymax - start_max;
                let run = (cycle.len() - idx) as i64;

                let cycle_block_count = block_count - idx as i64;
                let skip = dy * (cycle_block_count / run);

                let final_steps = (cycle_block_count % run) as usize;
                let final_skip = cycle[idx + final_steps].1 - start_max;

                println!(
                    "Skipped to {} + {} + {} + 1 = {}",
                    start_max,
                    skip,
                    final_skip,
                    start_max + skip + final_skip + 1
                );

                // no need to go on we printed the answer
                return;
            }
        }

        curr = offset_block(&BLOCKS[block_i], 2, ymax + 4);
        block_i = (block_i + 1) % BLOCKS.len();
        dump(&curr, &other);

        loop {
            //drop the block and fire jets until it rests
            if *DUMP {
                println!("{} {} {}", t, &INPUT[jet_i..=jet_i], jet_i);
            }
            t += 1;

            let xoff = match &INPUT[jet_i..=jet_i] {
                "<" => -1,
                ">" => 1,
                _ => panic!("Bad input"),
            };
            jet_i = (jet_i + 1) % INPUT.len();

            let next = offset_block(&curr, xoff, 0);
            if !check_wall(&next, &other) {
                curr = next;
            }

            let next = offset_block(&curr, 0, -1);

            if !check_below(&next, &other) {
                curr = next;
            } else {
                // block is now at rest, add points to the
                // surface and then get the new topology to
                // prune all the points below that are
                // unreachable
                other.extend(curr.iter().cloned());
                other = get_topo(&other);

                dump(&Block::new(), &other);

                break;
            }

            dump(&curr, &other);
        }

        bc += 1;
    }

    // get the top more point's y value and then add 1 since we're
    // tracking the "bottom" of the "pixel" and the problem wants
    // the top
    let ymax = *(&other.iter().map(|p| p.1.clone()).max().unwrap());
    println!("{}", ymax + 1);
}

fn dump(block: &Block, other: &Block) {
    if *DUMP {
        let ymax = other.iter().chain(block.iter()).map(|p| p.1).max().unwrap();

        for y in (0..=ymax).rev() {
            print!("|");
            for x in 0..=6 {
                if block.contains(&(x, y)) {
                    print!("@");
                } else if other.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("|");
        }
        println!("+-------+");
    }
}
