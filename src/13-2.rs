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
            .map(|n| n.as_str().parse::<i64>().unwrap())
            .collect();
        machines.push((
            nums[0],
            nums[1],
            nums[2],
            nums[3],
            nums[4] + 10000000000000,
            nums[5] + 10000000000000,
        ));
    }
    let mut acc: i64 = 0;
    for m in machines {
        let (x1, y1, x2, y2, r1, r2) = m;
        let d = (x1 * y2) - (y1 * x2);
        let a = (r1 * y2 - r2 * x2) / d;
        let b = (x1 * r2 - y1 * r1) / d;
        if (a >= 0) && (b >= 0) && (r1 == (x1 * a) + (x2 * b)) && (r2 == (y1 * a) + (y2 * b)) {
            acc += (3 * a) + b;
        }
    }
    println!("{}", acc);
}
