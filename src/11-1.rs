use std::fs::read_to_string;

fn trans(n: &u64) -> Vec<u64> {
    match n {
        0 => vec![1],
        n if n.to_string().chars().count() % 2 == 0 => {
            let d = 10_u32.pow((n.to_string().chars().count() / 2) as u32) as u64;
            let high = n / d;
            let low = n - (d * high);
            vec![high, low]
        }
        _ => vec![2024 * n],
    }
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut v = Vec::new();
    for i in f.split_whitespace() {
        if let Ok(n) = i.parse::<u64>() {
            v.push(n);
        }
    }
    for _ in 0..25 {
        v = v
            .into_iter()
            .map(|e| trans(&e))
            .collect::<Vec<_>>()
            .concat();
    }
    println!("{:?}", v.len());
}
