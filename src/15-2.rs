use std::cmp::min;
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
        distance.insert(i, 100000000000);
    }
    map[start] = 0;
    distance.insert(start, map[start]);
    let mut cur = start;
    while unvisited.contains(&end) {
        let curdis = *distance.get(&cur).unwrap();
        for i in neighbours(cur, l).iter().filter(|n| unvisited.contains(n)) {
            //println!("Checking {}.", i);
            let e = map[*i];
            let d = *distance.get(i).unwrap();
            //println!("Distance: {}", distance.get(i).unwrap());
            distance.insert(*i, min(curdis + e, d));
            //println!("Distance: {}", distance.get(i).unwrap());
        }
        unvisited.remove(&cur);
        cur = *unvisited
            .iter()
            .min_by(|a, b| distance.get(a).cmp(&distance.get(b)))
            .unwrap();
        //println!("Cur: {}.", cur);
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
