use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn north(&self) -> Self {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn south(&self) -> Self {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn west(&self) -> Self {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn east(&self) -> Self {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
    fn right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
        }
    }
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut start = Point { x: 0, y: 0 };
    let mut end = Point { x: 0, y: 0 };
    let mut walls: HashSet<Point> = HashSet::new();
    for (i, line) in f.split('\n').enumerate() {
        for (j, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    walls.insert(Point { x: i, y: j });
                }
                'S' => start = Point { x: i, y: j },
                'E' => end = Point { x: i, y: j },
                _ => (),
            }
        }
    }
    let mut visited: HashSet<(Point, Direction)> = HashSet::new();
    let mut unvisited = HashSet::new();
    let mut distance = HashMap::new();
    distance.insert((start, Direction::East), 0);
    unvisited.insert((start, Direction::East));
    let mut pred: HashMap<(Point, Direction), Vec<(Point, Direction)>> = HashMap::new();
    while let Some(node) = unvisited
        .iter()
        .min_by(|a, b| distance.get(a).cmp(&distance.get(b)))
    {
        let node = ((*node).clone(), distance.get(node).unwrap().clone());
        let mut neighbours: HashMap<(Point, Direction), usize> = HashMap::new();
        match node.0 .1 {
            Direction::North => {
                if !walls.contains(&node.0 .0.north()) {
                    neighbours.insert((node.0 .0.north(), node.0 .1), node.1 + 1);
                }
            }
            Direction::West => {
                if !walls.contains(&node.0 .0.west()) {
                    neighbours.insert((node.0 .0.west(), node.0 .1), node.1 + 1);
                }
            }
            Direction::South => {
                if !walls.contains(&node.0 .0.south()) {
                    neighbours.insert((node.0 .0.south(), node.0 .1), node.1 + 1);
                }
            }
            Direction::East => {
                if !walls.contains(&node.0 .0.east()) {
                    neighbours.insert((node.0 .0.east(), node.0 .1), node.1 + 1);
                }
            }
        }
        neighbours.insert((node.0 .0, node.0 .1.left()), node.1 + 1000);
        neighbours.insert((node.0 .0, node.0 .1.right()), node.1 + 1000);
        for n in neighbours {
            if !visited.contains(&n.0) {
                unvisited.insert(n.0);
                match distance.get(&n.0) {
                    None => {
                        distance.insert(n.0, n.1);
                        pred.insert(n.0, vec![node.0]);
                    }
                    Some(dist) if *dist > n.1 => {
                        pred.insert(n.0, vec![node.0]);
                        distance.insert(n.0, n.1);
                    }
                    Some(dist) if *dist == n.1 => {
                        pred.entry(n.0).and_modify(|v| v.extend(vec![node.0]));
                    }
                    Some(_) => (),
                };
            }
        }
        visited.insert(node.0);
        unvisited.remove(&node.0);
    }
    let mut visited: HashSet<Point> = HashSet::new();
    let mut endpoint = vec![distance
        .iter()
        .filter(|(k, _)| k.0 == end)
        .min_by_key(|(_, v)| *v)
        .unwrap()
        .0
        .clone()];
    visited.insert(end);
    while !visited.contains(&start) {
        endpoint = endpoint
            .iter()
            .filter_map(|e| pred.get(e))
            .flat_map(|e| e.iter())
            .cloned()
            .collect();
        endpoint.iter().for_each(|e| {
            visited.insert(e.0);
        });
    }
println!("{}", visited.len());
}
