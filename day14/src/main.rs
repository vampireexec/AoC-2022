#[macro_use]
extern crate lazy_static;
use std::{
    collections::BTreeMap, env::var, error::Error, fs::read_to_string, ops::Bound::Excluded,
};

use regex::Regex;

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref PAT: Regex = Regex::new(r"(\d+),(\d+)(?: -> )?").unwrap();
}

type Point = (i32, i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Surface {
    Start,
    Stop,
    Point,
}

#[derive(Debug, Clone)]
struct Fill {
    ranges: BTreeMap<i32, Surface>,
}
impl Fill {
    fn new() -> Self {
        Fill {
            ranges: BTreeMap::new(),
        }
    }

    fn contains(&self, x: i32) -> bool {
        match self.ranges.range(..=x).next_back() {
            Some((x0, kind)) => *kind == Surface::Start || *x0 == x,
            None => false,
        }
    }

    fn insert_point(&mut self, x: i32) {
        if let Some((x0, kind)) = self
            .ranges
            .range(..=x)
            .map(|(x0, s)| (x0.clone(), s.clone()))
            .next_back()
        {
            if kind == Surface::Start || x0 == x {
                return;
            }
        }

        let left_kind = self.ranges.get(&(x - 1)).and_then(|k| Some(k.clone()));
        let right_kind = self.ranges.get(&(x + 1)).and_then(|k| Some(k.clone()));

        match (left_kind, right_kind) {
            (Some(Surface::Stop), None) => {
                self.ranges.remove(&(x - 1));
                self.ranges.insert(x, Surface::Stop);
            }
            (Some(Surface::Stop), Some(Surface::Point)) => {
                self.ranges.remove(&(x - 1));
                self.ranges.remove(&(x + 1));
                self.ranges.insert(x + 1, Surface::Stop);
            }
            (Some(Surface::Stop), Some(Surface::Start)) => {
                self.ranges.remove(&(x - 1));
                self.ranges.remove(&(x + 1));
            }
            (Some(Surface::Point), None) => {
                self.ranges.insert(x - 1, Surface::Start);
                self.ranges.insert(x, Surface::Stop);
            }
            (Some(Surface::Point), Some(Surface::Point)) => {
                self.ranges.insert(x - 1, Surface::Start);
                self.ranges.insert(x + 1, Surface::Stop);
            }
            (None, Some(Surface::Point)) => {
                self.ranges.insert(x, Surface::Start);
                self.ranges.insert(x + 1, Surface::Stop);
            }
            (None, Some(Surface::Start)) => {
                self.ranges.insert(x, Surface::Start);
                self.ranges.remove(&(x + 1));
            }
            (Some(Surface::Point), Some(Surface::Start)) => {
                self.ranges.insert(x - 1, Surface::Start);
                self.ranges.remove(&(x + 1));
            }
            (None, None) => {
                self.ranges.insert(x, Surface::Point);
            }
            (e0, e1) => panic!(
                "Really shouldn't happen {:?} {:?}\n{:?}\n",
                e0, e1, self.ranges
            ),
        }
    }
}

type Cave = BTreeMap<i32, Fill>;

trait Physics {
    fn drop_sand(&mut self, sand: Point) -> Option<Point>;
}

impl Physics for Cave {
    fn drop_sand(&mut self, sand: Point) -> Option<Point> {
        let mut dest = sand;
        loop {
            let ent = self
                .range(dest.1..)
                .map(|(x, k)| (x.clone(), k.clone()))
                .next();
            if ent.is_none() {
                return None;
            }

            let (y, level) = ent.unwrap();
            if level.contains(dest.0) {
                if !level.contains(dest.0 - 1) {
                    dest = (dest.0 - 1, y);
                    continue;
                }

                if !level.contains(dest.0 + 1) {
                    dest = (dest.0 + 1, y);
                    continue;
                }

                self.entry(y - 1)
                    .or_insert(Fill::new())
                    .insert_point(dest.0);
                return Some((dest.0, y - 1));
            }
            dest = (dest.0, y + 1);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cave = Cave::new();
    for line in INPUT.lines() {
        let mut points = vec![];
        for p in PAT.captures_iter(line) {
            points.push((
                i32::from_str_radix(p.get(1).unwrap().as_str(), 10)?,
                i32::from_str_radix(p.get(2).unwrap().as_str(), 10)?,
            ));
        }
        for i in 1..points.len() {
            let xmin = points[i - 1].0.min(points[i].0);
            let xmax = points[i - 1].0.max(points[i].0);
            let ymin = points[i - 1].1.min(points[i].1);
            let ymax = points[i - 1].1.max(points[i].1);
            if xmin == xmax {
                for y in ymin..=ymax {
                    cave.entry(y).or_insert(Fill::new()).insert_point(xmin);
                }
            } else if ymin == ymax {
                let level = cave.entry(ymin).or_insert(Fill::new());
                if let Some((_, kind)) = level.ranges.range(..=xmax).next_back() {
                    if *kind != Surface::Start {
                        level.ranges.insert(xmax, Surface::Stop);
                    }
                } else {
                    level.ranges.insert(xmax, Surface::Stop);
                }

                if let Some((_, kind)) = level.ranges.range(..=xmin).next_back() {
                    if *kind != Surface::Start {
                        level.ranges.insert(xmin, Surface::Start);
                    }
                } else {
                    level.ranges.insert(xmin, Surface::Start);
                }

                let to_delete = level
                    .ranges
                    .range((Excluded(xmin), Excluded(xmax)))
                    .map(|p| p.0.clone())
                    .collect::<Vec<_>>();
                for k in to_delete {
                    level.ranges.remove(&k);
                }

                cave.entry(ymin)
                    .and_modify(|f| {
                        f.ranges.insert(xmin, Surface::Start);
                        f.ranges.insert(xmax, Surface::Stop);
                    })
                    .or_insert(Fill {
                        ranges: BTreeMap::from([(xmin, Surface::Start), (xmax, Surface::Stop)]),
                    });
            } else {
                panic!("Invalid input")
            }
        }
    }

    let mut p1 = 0;
    for i in 0.. {
        if cave.drop_sand((500, 0)).is_none() {
            println!("{}", i);
            p1 = i;
            break;
        }
    }
    cave.insert(
        cave.range(..).next_back().unwrap().0 + 2,
        Fill {
            ranges: BTreeMap::from([(i32::MIN, Surface::Start), (i32::MAX, Surface::Stop)]),
        },
    );

    for i in p1+1.. {
        cave.drop_sand((500, 0));
        if cave.contains_key(&0)
            && cave
                .get(&0)
                .and_then(|f| Some(f.ranges.contains_key(&500)))
                .unwrap()
        {
            println!("{}", i);
            break;
        }
    }

    for y in 0..=*cave.range(..).next_back().unwrap().0 {
        for x in 470..=540 {
            if let Some(level) = cave.get(&y) {
                if level.contains(x) {
                    print!("#");
                } else {
                    print!(".");
                }
            } else {
                print!(".");
            }
        }
        println!("");
    }
    Ok(())
}
