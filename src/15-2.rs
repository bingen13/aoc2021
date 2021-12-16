use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let mut map: Vec<usize> = Vec::new();
    let mut l = 0;
    for i in f {
        if i.is_empty() {
            break;
        }
        l = i.len();
        for j in i.chars() {
            map.push(j.to_digit(10).unwrap().try_into().unwrap());
        }
    }
    map = step(map, l);
    l *= 5;
    let start = 0;
    let end = map.len() - 1;
    let mut distance = HashMap::new();
    let mut unvisited = HashSet::new();
    for i in 0..map.len() {
        unvisited.insert(i);
    }
    distance.insert(start, 0);
    while !distance.contains_key(&end) {
        let cur = distance
            .iter()
            .filter(|a: &(&usize, &usize)| unvisited.contains(a.0))
            .min_by(|a, b| a.1.cmp(b.1))
            .unwrap();
        let curdis = *cur.1;
        let cur = *cur.0;
        for i in neighbours(cur, l).iter() {
            if unvisited.contains(i) {
                let e = map[*i];
                if let Some(d) = distance.get(i) {
                    if d > &(curdis + e) {
                        distance.insert(*i, curdis + e);
                    }
                } else {
                    distance.insert(*i, curdis + e);
                }
            }
        }
        unvisited.remove(&cur);
    }
    println!("{:?}", distance.get(&end).unwrap());
    /*
        for (i, j) in map.iter().enumerate() {
            print!("{}", j);
            if i % 50 == 49 {
                print!("\n");
            }
        }
    */
}

fn neighbours(x: usize, l: usize) -> Vec<usize> {
    let mut n = Vec::new();
/*
    if x > l {
        n.push(x - l);
    }
*/
    if x < l * (l - 1) {
        n.push(x + l);
    }
/*
    if x % l > 0 {
        n.push(x - 1);
    }
*/
    if x % l < (l - 1) {
        n.push(x + 1);
    }
    n
}

fn step(m: Vec<usize>, l: usize) -> Vec<usize> {
    let mut n = Vec::new();
    for i in 0..25 * (l * l) {
        let x = i % l;
        let y = (i / (5 * l)) % l;
        n.push(m[x + (l * y)]);
    }
    for (i, j) in n.iter_mut().enumerate() {
        let x = (i / l) % 5;
        let y = i / (5 * l * l) % 5;
        *j += x + y;
        if j > &mut 9 {
            *j = (*j % 10) + 1;
        }
    }
    n
}
