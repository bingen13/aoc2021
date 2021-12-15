use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n");
    let mut map: Vec<usize> = Vec::new();
    let mut l = 0;
    for i in f {
        if i.len() == 0 {
            break;
        }
        l = i.len();
        for j in i.chars() {
            map.push(j.to_digit(10).unwrap().try_into().unwrap());
        }
    }
    let start = 0;
    let end = map.len() - 1;
    let mut total = 0;
    let mut distance = HashMap::new();
    let mut visited = HashSet::new();
    distance.insert(start, 0);
    while !distance.contains_key(&end) {
        let cur = distance
            .iter()
            .filter(|a: &(&usize, &usize)| !visited.contains(a.0))
            .min_by(|a, b| a.1.cmp(b.1))
            .unwrap()
            .clone();
        let curdis = cur.1.clone();
        let cur = cur.0.clone();
        for i in neighbours(cur, l).iter() {
            if !visited.contains(i) {
                let e = map[*i];
                if let Some(d) = distance.get(&i) {
                    if d > &(curdis + e) {
                        distance.insert(*i, curdis + e);
                    }
                } else {
                    distance.insert(*i, curdis + e);
                }
            }
        }
        visited.insert(cur);
    }
    println!("{:?}", distance.get(&end).unwrap());
}

fn neighbours(x: usize, l: usize) -> Vec<usize> {
    let mut n = Vec::new();
    if x > l {
        n.push(x - l);
    }
    if x < l * (l - 1) {
        n.push(x + l);
    }
    if x % l > 0 {
        n.push(x - 1);
    }
    if x % l < (l - 1) {
        n.push(x + 1);
    }
    n
}

fn step(m: Vec<usize>) -> Vec<usize> {
    let mut n = Vec::new();
    for i in m.iter() {
        n.push((i + 1) % 10);
    }
    n
}
