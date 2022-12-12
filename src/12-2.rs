use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn neighbour(x: u32, y: u32) -> Vec<(u32, u32)> {
    let mut v = Vec::new();
    match (x, y) {
        (0, 0) => {
            v.push((0, 1));
            v.push((1, 0));
        }
        (0, 1..) => {
            v.push((0, y - 1));
            v.push((0, y + 1));
            v.push((1, y));
        }
        (1.., 0) => {
            v.push((x - 1, 0));
            v.push((x + 1, 0));
            v.push((x, 1));
        }
        (1.., 1..) => {
            v.push((x - 1, y));
            v.push((x + 1, y));
            v.push((x, y - 1));
            v.push((x, y + 1));
        }
    }
    v
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut x = 0;
    let mut y = 0;
    let mut map = HashMap::new();
    let mut end = (0, 0);
    for i in f.chars() {
        if i == '\n' {
            x += 1;
            y = 0;
        } else {
            if ('a'..='z').contains(&i) {
                map.insert((x, y), i as u32 - 'a' as u32);
            } else if i == 'S' {
                map.insert((x, y), 0);
            } else if i == 'E' {
                end = (x, y);
                map.insert((x, y), 26);
            }
            y += 1;
        }
    }
    let mut visit = vec![end];
    let mut visited = HashSet::new();
    let mut steps = 0;
    'major: loop {
        let mut newq = Vec::new();
        for i in visit {
            for j in neighbour(i.0, i.1) {
                if !visited.contains(&j)
                    && map.contains_key(&j)
                    && (&(map.get(&j).unwrap() + 1) >= map.get(&i).unwrap())
                {
                    visited.insert(j);
                    newq.push(j);
                    if map.get(&j).unwrap() == &0 {
                        steps += 1;
                        break 'major;
                    }
                }
            }
        }
        visit = newq;
        steps += 1;
    }
    println!("{}", steps);
}
