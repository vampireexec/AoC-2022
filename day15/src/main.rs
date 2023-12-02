#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::{collections::HashSet, env::var, error::Error, fs::read_to_string};

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref PAT: Regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
}

#[derive(Debug, Clone, Copy)]
struct Sensor {
    loc: (i32, i32),
    beacon: (i32, i32),
    ds: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut sensors = vec![];
    for line in INPUT.lines() {
        let cap = PAT.captures(line).unwrap();
        let loc = (
            i32::from_str_radix(cap.get(1).unwrap().as_str(), 10)?,
            i32::from_str_radix(cap.get(2).unwrap().as_str(), 10)?,
        );
        let beacon = (
            i32::from_str_radix(cap.get(3).unwrap().as_str(), 10)?,
            i32::from_str_radix(cap.get(4).unwrap().as_str(), 10)?,
        );
        let ds = (beacon.0 - loc.0).abs() + (beacon.1 - loc.1).abs();
        sensors.push(Sensor { loc, beacon, ds });
    }

    let max = i32::from_str_radix(var("MAX").unwrap().as_str(), 10).unwrap();
    let target_y = i32::from_str_radix(var("TARG_Y").unwrap().as_str(), 10).unwrap();

    //Part 1
    let mut covered = HashSet::new();
    let mut beacons = HashSet::new();
    for s in &sensors {
        let dtarget = s.ds - (target_y - s.loc.1).abs();
        if dtarget > 0 {
            for i in (s.loc.0 - dtarget)..=(s.loc.0 + dtarget) {
                covered.insert(i);
            }
        }
        if s.beacon.1 == target_y {
            beacons.insert(s.beacon.0);
        }
    }
    println!("{}", covered.len() - beacons.len());

    //Part 2
    for i in 0..sensors.len() {
        let b0 = &sensors[i];
        let ymin = (b0.loc.1 - b0.ds - 1).clamp(0, max);
        let ymax = (b0.loc.1 + b0.ds + 1).clamp(0, max);
        let mut check = vec![];
        for y in ymin..ymax {
            let dx = b0.ds + 1 - (y - b0.loc.1).abs();
            if b0.loc.0 + dx > 0 && b0.loc.0 + dx < max {
                check.push((b0.loc.0 + dx, y));
            }
            if b0.loc.0 - dx > 0 && b0.loc.0 - dx < max {
                check.push((b0.loc.0 - dx, y));
            }
        }
        let mut found = HashSet::new();
        for j in 0..sensors.len() {
            if i == j {
                continue;
            }
            let b1 = &sensors[j];
            for c in &check {
                let ds_c = (c.0 - b1.loc.0).abs() + (c.1 - b1.loc.1).abs();
                if ds_c <= b1.ds {
                    found.insert(*c);
                }
            }
        }
        let orig = HashSet::from_iter(check);
        let dif = orig.difference(&found).collect::<Vec<_>>();
        if dif.len() > 0 {
            let x = dif[0].0;
            let y = dif[0].1;
            println!("{},{} = {}", x, y, x as u64 * 4000000 as u64 + y as u64);
            return Ok(());
        }
    }

    Ok(())
}
