use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n");
    let mut points = HashMap::new();
    for i in f {
        if i.len() == 0 {
            break;
        }
        let mut line = i.split(" -> ");
        let mut origin = line.next().unwrap().split(",");
        let (x1, y1) = (
            origin.next().unwrap().parse::<u32>().unwrap(),
            origin.next().unwrap().parse::<u32>().unwrap(),
        );
        let mut target = line.next().unwrap().split(",");
        let (x2, y2) = (
            target.next().unwrap().parse::<u32>().unwrap(),
            target.next().unwrap().parse::<u32>().unwrap(),
        );
        if x1 == x2 {
            for j in min(y1, y2)..max(y1, y2) + 1 {
                *points.entry(Point { x: x1, y: j }).or_insert(0) += 1;
            }
        }
        if y1 == y2 {
            for j in min(x1, x2)..max(x1, x2) + 1 {
                *points.entry(Point { x: j, y: y1 }).or_insert(0) += 1;
            }
        }
    }
    let mut overlaps = 0;
    for (_, j) in points {
        if j > 1 {
            overlaps += 1;
        }
    }
    println!("{}", overlaps);
}
