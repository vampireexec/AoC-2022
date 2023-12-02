#[macro_use]
extern crate lazy_static;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    env::args,
    fmt::Display,
    fs::read_to_string, thread,
};

lazy_static! {
    static ref INPUT: String = read_to_string(args().nth(1).unwrap()).unwrap();
}

type Point = (i32, i32);

#[derive(Clone,Copy)]
enum Location {
    Start(),
    End(),
    Elevation(i32),
}

impl Location {
    fn height(&self) -> i32 {
        match self {
            Location::Start() => b'a' as i32,
            Location::End() => b'z' as i32,
            Location::Elevation(x) => *x,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Location::Start() => 'S',
            Location::End() => 'E',
            Location::Elevation(x) => char::from_u32(x.unsigned_abs()).unwrap(),
        }
    }

    fn is_end(&self) -> bool {
        match self {
            Location::End() => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
struct TopoMap {
    start: Point,
    topo: HashMap<Point, Location>,
    mins: Vec<Point>,
}

impl TopoMap {
    fn from_input() -> Self {
        let mut topo: HashMap<Point, Location> = HashMap::new();
        let mut start: Point = (-1, -1);
        let mut mins = vec![];
        for (y, line) in (*INPUT).lines().enumerate() {
            for (x, h) in line.bytes().enumerate() {
                let p = (x as i32, y as i32);
                if h == b'S' {
                    start = p;
                    mins.push(p);
                    topo.insert(p, Location::Start());
                } else if h == b'E' {
                    topo.insert(p, Location::End());
                } else {
                    if h == b'a' {
                        mins.push(p);
                    }
                    topo.insert(p, Location::Elevation(h as i32));
                }
            }
        }
        Self { start, topo, mins }
    }

    fn bfs(&self, start: Point) -> i32 {
        let mut que = VecDeque::from([(0, start)]);
        let mut visited: HashSet<Point> = HashSet::from([start]);

        while !que.is_empty() {
            let (step, curr) = que.pop_front().unwrap();
            let cl = self.topo.get(&curr).unwrap();
            if cl.is_end() {
                return step;
            }

            for ds in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let next = (curr.0 + ds.0, curr.1 + ds.1);
                if visited.contains(&next) || !self.topo.contains_key(&next) {
                    continue;
                }
                let nl = self.topo.get(&next).unwrap();
                if cl.height() - nl.height() >= -1 {
                    visited.insert(next);
                    que.push_back((step + 1, next));
                }
            }
        }
        return i32::MAX;
    }
}

fn main() {
    let topo_map = TopoMap::from_input();
    println!("{}", topo_map.bfs(topo_map.start));

    let mut handles = vec![];
    for start in topo_map.mins.iter().cloned() {
        let topo_clone = topo_map.clone();
        handles.push(thread::spawn(move || topo_clone.bfs(start)));
    }
    
    let mut paths = vec![];
    for h in handles {
        paths.push(h.join().unwrap());
    }
    paths.sort();
    println!("{}", paths.first().unwrap());
}

impl Display for TopoMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let xmax = self.topo.keys().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
        let ymax = self.topo.keys().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
        for i in 0..ymax {
            for j in 0..xmax {
                write!(f, "{}", self.topo.get(&(i, j)).unwrap().to_char())?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}