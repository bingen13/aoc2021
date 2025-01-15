use std::fs::read_to_string;
use std::iter::successors;

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
    let suc = |n: &i64| Some(next_secret(*n));
    println!(
        "{}",
        numbers
            .iter()
            .filter_map(|&n| successors(Some(n), suc).nth(2000))
            .sum::<i64>()
    );
}
