use std::fs::read_to_string;

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
    'count: for i in numbers {
        let mut v = Vec::new();
        for j in 0..(i.len() - 1) {
            let diff = i[j] - i[j + 1];
            if (diff.abs() > 3) | (diff == 0) {
                continue 'count;
            }
            v.push(diff);
        }
        if v.iter().all(|n| *n > 0) | v.iter().all(|n| *n < 0) {
            safe += 1;
        }
    }
    println!("{}", safe);
}
