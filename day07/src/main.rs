use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

fn input_txt() -> String {
    let path = env::args().nth(1).unwrap_or(String::from("ex1.txt"));
    fs::read_to_string(path).ok().unwrap_or_default()
}

fn main() {
    let input = input_txt();
    part12(&input);
}

fn canonicalize(path: &str) -> Vec<&str> {
    let mut ret: Vec<&str> = vec![];
    for c in path.split("/") {
        match c {
            "" => ret.push("/"),
            ".." => _ = ret.pop(),
            "." => (),
            _ => ret.push(c),
        }
    }
    ret
}

fn part12(input: &String) {
    let mut cwd: PathBuf = PathBuf::new();
    let mut sizes: HashMap<String, u32> = HashMap::new();

    // Some short cuts here but this works because all we care about
    // is the cwd and any file sizes we encounter at that cwd
    for line in input.lines() {
        if line.starts_with("$ cd") {
            if &line[5..6] == "/" {
                cwd.clear();
            } else if &line[5..] == ".." || (&line[5..]).starts_with("../") {
                cwd.pop();
            }
            cwd.extend(canonicalize(&line[5..]));
        } else if line.as_bytes()[0].is_ascii_digit() {
            let ent: Vec<&str> = line.split(" ").collect();
            sizes.insert(
                cwd.join(ent[1]).display().to_string(),
                u32::from_str_radix(ent[0], 10).unwrap(),
            );
        }
    }

    // for each file, add its size to each directory before it
    let mut sums: HashMap<String, u32> = HashMap::new();
    for (f, s) in &sizes {
        for p in Path::new(f).ancestors().skip(1) {
            sums.entry(p.display().to_string())
                .and_modify(|x| *x += s)
                .or_insert(*s);
        }
    }

    println!("{:?}", sums.values().filter(|s| **s <= 100000).sum::<u32>());

    let total = sizes.values().sum::<u32>();
    let target = total - (70000000 - 30000000);
    let mut sorted: Vec<(&u32, &String)> = sums.iter().map(|(k, v)| (v, k)).collect();
    sorted.sort();
    for (v, _) in sorted {
        if *v >= target {
            println!("{}", v);
            break;
        }
    }
}
