use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs::read_to_string;
use std::io::stdout;
use std::io::Write;

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
    let mut visited = HashSet::new();
    map[start] = 0;
    distance.insert(start, map[start]);
    let mut cur = start;
    while !distance.contains_key(&end)  {
        let curdis = *distance.get(&cur).unwrap();
        for i in [north(cur, l), south(cur, l), west(cur, l), east(cur, l)]
            .iter()
            .flatten()
            .filter(|n| !visited.contains(*n))
        {
            let e = map[*i];
            let d = *distance.get(i).unwrap_or(&100000000000);
            if curdis + e < d {
                distance.insert(*i, curdis + e);
                unvisited.insert(*i);
            }
        }
        unvisited.remove(&cur);
        visited.insert(cur);
        cur = *unvisited
            .iter()
            .min_by(|a, b| distance.get(a).cmp(&distance.get(b)))
            .unwrap();
    }
    println!("{:?}", distance.get(&end).unwrap());
}

fn north(x: usize, l: usize) -> Option<usize> {
    if x > l {
        Some(x - l)
    } else {
        None
    }
}

fn south(x: usize, l: usize) -> Option<usize> {
    if x < l * (l - 1) {
        Some(x + l)
    } else {
        None
    }
}

fn west(x: usize, l: usize) -> Option<usize> {
    if x % l > 0 {
        Some(x - 1)
    } else {
        None
    }
}

fn east(x: usize, l: usize) -> Option<usize> {
    if x % l < (l - 1) {
        Some(x + 1)
    } else {
        None
    }
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
