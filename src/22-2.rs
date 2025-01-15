use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::{once, successors};

fn mix(n1: i64, n2: i64) -> i64 {
    n2 ^ n1
}

fn prune(n: i64) -> i64 {
    n % 16777216
}

fn next_secret(n: i64) -> i64 {
    let mut secret = mix(n * 64, n);
    secret = prune(secret);
    secret = mix(secret / 32, secret);
    secret = prune(secret);
    secret = mix(secret * 2048, secret);
    secret = prune(secret);
    secret
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let numbers: Vec<_> = f.lines().map(|l| l.parse::<i64>().unwrap()).collect();
    let mut v = Vec::new();
    let suc = |n: &i64| Some(next_secret(*n));
    numbers.iter().for_each(|&n| {
        let mut h: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
        once(n)
            .chain(successors(Some(n), suc).take(2000))
            .map(|n| n % 10)
            .collect::<Vec<_>>()
            .windows(5)
            .for_each(|w| {
                h.entry((w[1] - w[0], w[2] - w[1], w[3] - w[2], w[4] - w[3]))
                    .or_insert(w[4]);
            });
        v.push(h);
    });
    let mut h = HashMap::new();
    v.iter()
        .for_each(|m| m.iter().for_each(|(k, v)| *h.entry(k).or_insert(0) += v));
    println!("{:?}", h.iter().max_by_key(|(_, v)| *v).unwrap());
println!("{}", numbers.len());
}
