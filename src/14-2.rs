use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut f = f.split("\n\n");
    let start = f.next().unwrap().to_string().chars().collect::<Vec<char>>();
    let mut transforms = HashMap::new();
    for i in f.next().unwrap().split("\n") {
        if i.len() == 0 {
            break;
        }
        let mut j = i.split(" -> ");
        let origin = j.next().unwrap().to_string();
        let target = j.next().unwrap();
        let mut origin = origin.chars();
        let mut target = target.chars();
        let c1 = origin.next().unwrap();
        let c2 = origin.next().unwrap();
        let c3 = target.next().unwrap();
        transforms.insert((c1, c2), c3);
    }
    let first = start[0];
    let mut pairs = HashMap::new();
    for i in start[..].windows(2) {
        let c1 = i[0];
        let c2 = i[1];
        *pairs.entry((c1, c2)).or_insert(0) += 1;
    }
    for _ in 0..40 {
        pairs = step(pairs, &transforms);
    }
    let mut results = HashMap::new();
    results.insert(first, 1);
    for i in pairs {
        *results.entry(i.0 .1).or_insert(0) += i.1;
    }
    println!(
        "{:?}",
        results.values().max().unwrap() - results.values().min().unwrap()
    );
}

fn step(
    h: HashMap<(char, char), u64>,
    t: &HashMap<(char, char), char>,
) -> HashMap<(char, char), u64> {
    let mut n = HashMap::new();
    for p in h {
        if let Some(c) = t.get(&p.0) {
            *n.entry((p.0 .0, *c)).or_insert(0) += p.1;
            *n.entry((*c, p.0 .1)).or_insert(0) += p.1;
        }
    }
    n
}
