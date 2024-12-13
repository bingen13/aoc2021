use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let r = Regex::new(r"\d+").unwrap();
    let mut machines = Vec::new();
    for i in f.split("\n\n") {
        if i.is_empty() {
            break;
        }
        let nums: Vec<_> = r
            .find_iter(i)
            .map(|n| n.as_str().parse::<u32>().unwrap())
            .collect();
        machines.push((nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]));
    }
    let mut cost = Vec::new();
    for (n, m) in machines.iter().enumerate() {
        for i in 0..=100 {
            for j in 0..=100 {
                let x = (i * m.0) + (j * m.2);
                let y = (i * m.1) + (j * m.3);
                if (x == m.4) && (y == m.5) {
                    cost.push((n, i, j, (3 * i) + j));
                }
            }
        }
        let mincost = cost.iter().filter(|e| e.0 == n).map(|e| e.3).min();
        if let Some(minc) = mincost {
            cost.retain(|e| (e.0 != n) || (e.3 == minc));
        }
    }
    let mut acc = 0;
    for n in 0..machines.len() {
        if let Some(c) = cost.iter().find(|e| e.0 == n) {
            acc += c.3;
        }
    }
    println!("{}", acc);
}
