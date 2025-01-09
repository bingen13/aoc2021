use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn north(&self) -> Option<Self> {
        if self.y > 0 {
            Some(Point {
                x: self.x,
                y: self.y - 1,
            })
        } else {
            None
        }
    }
    fn south(&self) -> Option<Self> {
        if self.y < 140 {
            Some(Point {
                x: self.x,
                y: self.y + 1,
            })
        } else {
            None
        }
    }
    fn west(&self) -> Option<Self> {
        if self.x > 0 {
            Some(Point {
                x: self.x - 1,
                y: self.y,
            })
        } else {
            None
        }
    }
    fn east(&self) -> Option<Self> {
        if self.x < 140 {
            Some(Point {
                x: self.x + 1,
                y: self.y,
            })
        } else {
            None
        }
    }
}

fn dist(p1: &Point, p2: &Point) -> usize {
    (max(p1.x, p2.x) - min(p1.x, p2.x)) + (max(p1.y, p2.y) - min(p1.y, p2.y))
}

fn length(
    start: &Point,
    walls: &HashSet<Point>,
    distance: &mut HashMap<Point, usize>,
    pred: &mut HashMap<Point, Point>,
) {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut unvisited = HashSet::new();
    distance.insert(*start, 0);
    unvisited.insert(*start);
    while let Some(node) = unvisited
        .iter()
        .min_by(|a, b| distance.get(a).cmp(&distance.get(b)))
    {
        let node = ((*node), *distance.get(node).unwrap());
        let neighbours = [node.0.north(), node.0.south(), node.0.east(), node.0.west()]
            .into_iter()
            .flatten()
            .map(|x| (x, node.1 + 1))
            .filter(|x| !walls.contains(&x.0))
            .collect::<Vec<_>>();
        for n in neighbours.into_iter() {
            if !visited.contains(&n.0) {
                unvisited.insert(n.0);
                match distance.get(&n.0) {
                    None => {
                        pred.insert(n.0, node.0);
                        distance.insert(n.0, n.1);
                    }
                    Some(dist) if *dist > n.1 => {
                        pred.insert(n.0, node.0);
                        distance.insert(n.0, n.1);
                    }
                    Some(_) => (),
                };
            }
        }
        visited.insert(node.0);
        unvisited.remove(&node.0);
    }
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut start = Point { x: 0, y: 0 };
    let mut end = Point { x: 70, y: 70 };
    let mut walls: HashSet<Point> = HashSet::new();
    for (i, line) in f.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    walls.insert(Point { x: j, y: i });
                }
                '.' => (),
                'S' => start = Point { x: j, y: i },
                'E' => end = Point { x: j, y: i },
                _ => unimplemented!(),
            }
        }
    }
    let mut distance = HashMap::new();
    let mut pred: HashMap<Point, Point> = HashMap::new();
    length(&end, &walls, &mut distance, &mut pred);
    let mut path = Vec::new();
    path.push(start);
    while let Some(n) = pred.get(path.last().unwrap()) {
        path.push(*n);
    }
    let mut c = 0;
    for n in 2..=20 {
        for p in &path {
            let d = *distance.get(p).unwrap();
            if d < 100 {
                continue;
            }
            let cans = distance.iter().filter(|(k, _)| dist(p, k) == n);
            for bestplace in cans {
                let mindist = dist(p, bestplace.0) + bestplace.1;
                if (mindist < d) && (d - mindist >= 100) {
                    c += 1;
                }
            }
        }
    }
    println!("{}", c);
}
