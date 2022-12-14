use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

fn draw_line(p1: Point, p2: Point, map: &mut HashSet<Point>) {
    if p1.x == p2.x {
        let x = p1.x;
        for i in min(p1.y, p2.y)..=max(p1.y, p2.y) {
            map.insert(Point { x, y: i });
        }
    } else if p1.y == p2.y {
        let y = p1.y;
        for i in min(p1.x, p2.x)..=max(p1.x, p2.x) {
            map.insert(Point { x: i, y });
        }
    }
}

fn place_sand(map: &mut HashSet<Point>, lowest: u32) -> bool {
    let mut x = 500;
    let mut y = 0;
    while y <= lowest {
        let (left, down, right) = (
            Point { x: x - 1, y: y + 1 },
            Point { x, y: y + 1 },
            Point { x: x + 1, y: y + 1 },
        );
        if !map.contains(&down) {
            y += 1;
        } else if !map.contains(&left) {
            y += 1;
            x -= 1;
        } else if !map.contains(&right) {
            y += 1;
            x += 1;
        } else {
            map.insert(Point { x, y });
            return true;
        }
    }
    false
}

fn main() {
    let re = Regex::new(r"(\d*),(\d*)").unwrap();
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n");
    let mut rocks: HashSet<Point> = HashSet::new();
    for i in f {
        let mut prev: Option<Point> = None;
        for j in re.captures_iter(i) {
            let p = Point {
                x: j[1].parse::<u32>().unwrap(),
                y: j[2].parse::<u32>().unwrap(),
            };
            if let Some(p2) = prev {
                draw_line(p2, p, &mut rocks);
            }
            prev = Some(p);
        }
    }
    // Find the lowest unit of rock.
    let lowest = rocks.iter().max_by_key(|p| p.y).unwrap().y;
    let mut grains = 0;
    while place_sand(&mut rocks, lowest) {
        grains += 1;
    }
    println!("{}", grains);
}
