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
        if self.y < 70 {
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
        if self.x < 70 {
            Some(Point {
                x: self.x + 1,
                y: self.y,
            })
        } else {
            None
        }
    }
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let start = Point { x: 0, y: 0 };
    let end = Point { x: 70, y: 70 };
    let mut walls: HashSet<Point> = HashSet::new();
    let mut obst = Vec::new();
    for i in f.split('\n') {
        if i.is_empty() {
            break;
        }
        let mut i = i.split(',');
        obst.push((
            i.next().unwrap().parse::<usize>().unwrap(),
            i.next().unwrap().parse::<usize>().unwrap(),
        ));
    }
    for i in 0..1024 {
        walls.insert(Point {
            x: obst[i].0,
            y: obst[i].1,
        });
    }
    let mut visited: HashSet<Point> = HashSet::new();
    let mut unvisited = HashSet::new();
    let mut distance = HashMap::new();
    distance.insert(start, 0);
    unvisited.insert(start);
    while let Some(node) = unvisited
        .iter()
        .min_by(|a, b| distance.get(a).cmp(&distance.get(b)))
    {
        let node = ((*node).clone(), distance.get(node).unwrap().clone());
        if node.0 == end {
            println!("{}", node.1);
            break;
        }
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
                        distance.insert(n.0, n.1);
                    }
                    Some(dist) if *dist > n.1 => {
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
