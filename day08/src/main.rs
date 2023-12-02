use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

fn input_txt() -> String {
    let path = env::args().nth(1).unwrap_or(String::from("ex1.txt"));
    fs::read_to_string(path).ok().unwrap_or_default()
}

fn main() {
    let input = input_txt();
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut trees: HashMap<(usize, usize), u32> = HashMap::new();
    let mut maxi = 0;
    let mut maxj = 0;

    for line in input.lines() {
        maxj = 0;
        for h in line.as_bytes() {
            trees.insert((maxi, maxj), (*h - b'0') as u32);
            maxj += 1;
        }
        maxi += 1;
    }

    let mut vis: HashSet<(usize, usize)> = HashSet::new();
    //cols
    for i in 0..maxi {
        let mut cur = trees.get(&(i, 0)).unwrap();
        vis.insert((i, 0));
        for j in 1..maxj {
            if trees.get(&(i, j)).unwrap() > cur {
                cur = trees.get(&(i, j)).unwrap();
                vis.insert((i, j));
            }
        }
    }

    for i in 0..maxi {
        let mut cur = trees.get(&(i, maxj-1)).unwrap();
        vis.insert((i, maxj-1));
        for j in (0..maxj-1).rev(){
            if trees.get(&(i, j)).unwrap() > cur {
                cur = trees.get(&(i, j)).unwrap();
                vis.insert((i, j));
            }
        }
    }
    //rows
    for j in 0..maxj {
        let mut cur = trees.get(&(0, j)).unwrap();
        vis.insert((0, j));
        for i in 1..maxi {
            if trees.get(&(i, j)).unwrap() > cur {
                cur = trees.get(&(i, j)).unwrap();
                vis.insert((i, j));
            }
        }
    }
    for j in 0..maxj {
        let mut cur = trees.get(&(maxi-1, j)).unwrap();
        vis.insert((maxi-1, j));
        for i in (0..maxi-1).rev() {
            if trees.get(&(i, j)).unwrap() > cur {
                cur = trees.get(&(i, j)).unwrap();
                vis.insert((i, j));
            }
        }
    }
    println!("{}", vis.len());
    // let mut s : Vec<(usize, usize)> = Vec::from_iter(vis);
    // s.sort();
    // for i in 0..maxi {
    //     for j in 0..maxj {
    //         if vis.contains(&(i,j)) {
    //             print!("*");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!("");
    // }
    //println!("{:?}", s);
    
}

fn part2(input: &String) {
    let mut trees: HashMap<(usize, usize), u32> = HashMap::new();
    let mut maxi = 0;
    let mut maxj = 0;

    for line in input.lines() {
        maxj = 0;
        for h in line.as_bytes() {
            trees.insert((maxi, maxj), (*h - b'0') as u32);
            maxj += 1;
        }
        maxi += 1;
    }

    let mut scores : Vec<(u32,(u32, u32, u32, u32), usize,usize)> = Vec::new();
    for i in 1..maxi {
        for j in 1..maxj {
            let t = trees.get(&(i, j)).unwrap();
            let mut s = 1;
            let mut v = 0;
            let mut r = (0, 0, 0, 0);
            for x in (0..=j-1).rev() {
                // println!("{} {}", trees.get(&(i,x)).unwrap(), t);
                if trees.get(&(i,x)).unwrap() < t {
                    // println!("here");
                    v+=1;
                } else {
                    v+=1;
                    break;
                }
            }
            r.0 = v;
            s *= v;
            v = 0;
        
            // println!("{}", s);
            for x in j+1..maxj {
                if trees.get(&(i,x)).unwrap() < t {
                    v+=1;
                } else {
                    v+=1;
                    break;
                }
            }
            r.1 = v;
            s *= v;
            v = 0;
            // println!("{}", s);

            for x in (0..=i-1).rev() {
                if trees.get(&(x,j)).unwrap() < t {
                    v+=1;
                } else {
                    v+=1;
                    break;
                }
            }
        
            r.2 = v;
            s *= v;
            v = 0;
        
            // println!("{}", s);

            for x in i+1..maxi {
                if trees.get(&(x,j)).unwrap() < t {
                    v+=1;
                } else {
                    v+=1;
                    break;
                }
            }
            r.3 = v;
            s *= v;
            v = 0;
            scores.push((s, r, i, j));
        }
    }

    scores.sort();
    scores.reverse();
    println!("{:?}", scores[0]);
}
