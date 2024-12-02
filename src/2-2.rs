use std::fs::read_to_string;

fn verify(v: &[i32]) -> bool {
    let mut d = Vec::new();
    for i in 0..(v.len() - 1) {
        let diff = v[i] - v[i + 1];
        if (diff == 0) | (diff.abs() > 3) {
            return false;
        }
        d.push(diff);
    }
    d.iter().all(|n| n.signum() == d[0].signum())
}

fn main() {
    let mut numbers: Vec<Vec<i32>> = Vec::new();
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    for i in f {
        if i.is_empty() {
            break;
        }
        let v: Vec<i32> = i
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        numbers.push(v);
    }
    let mut safe = 0;
    'numbers: for i in numbers {
        if verify(&i) {
            safe += 1;
            continue;
        }
        for j in 0..i.len() {
            let mut v = i.clone();
            v.remove(j);
            if verify(&v) {
                safe += 1;
                continue 'numbers;
            }
        }
    }
    println!("{}", safe);
}
