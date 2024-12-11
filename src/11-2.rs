use std::collections::HashMap;
use std::fs::read_to_string;

fn trans(n: &i64) -> Vec<i64> {
    match n {
        0 => vec![1],
        n if n.to_string().chars().count() % 2 == 0 => {
            let d = 10_u32.pow((n.to_string().chars().count() / 2) as u32) as i64;
            let high = n / d;
            let low = n - (d * high);
            vec![high, low]
        }
        _ => vec![2024 * n],
    }
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut m = HashMap::new();
    for i in f.split_whitespace() {
        if let Ok(n) = i.parse::<i64>() {
            m.insert(n, 1);
        }
    }
    let mut m2 = HashMap::new();
    for _ in 0..75 {
        for (k, val) in m {
            let v = trans(&k);
            for i in v {
                *m2.entry(i).or_insert(0) += val;
            }
        }
        m = m2.clone();
        m2.clear();
    }
    println!("{}", m.values().sum::<i64>());
}
