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
    return v;
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut x = 0;
    let mut y = 0;
    let mut map = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in f.chars() {
        if i == '\n' {
            x += 1;
            y = 0;
        } else {
            if (('a' as u32)..=('z' as u32)).contains(&(i as u32)) {
                map.insert((x, y), (i as u32 - 'a' as u32));
            } else if i == 'S' {
                start = (x, y);
                map.insert((x, y), 0);
            } else if i == 'E' {
                end = (x, y);
                map.insert((x, y), 26);
            }
            y += 1;
        }
    }
    let mut visit = Vec::new();
    visit.push(start);
    let mut visited = HashSet::new();
    let mut steps = 0;
    while !visited.contains(&end) {
        let mut newq = Vec::new();
        for i in visit {
            for j in neighbour(i.0, i.1) {
                if !visited.contains(&j)
                    && map.contains_key(&j)
                    && (map.get(&j).unwrap() <= &(map.get(&i).unwrap() + 1))
                {
                    visited.insert(j);
                    newq.push(j);
                }
            }
        }
        visit = newq;
        steps += 1;
    }
    println!("{}", steps);
}
