use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n");
    let mut numbers = Vec::new();
    let r = Regex::new(r"(\d*)").unwrap();
    for s in f {
        for i in r.captures_iter(s) {
            if let Ok(n) = i[1].parse::<u64>() {
                numbers.push(n);
            }
        }
    }
    for i in 0..numbers.len() - 25 {
        let target = numbers[i + 25];
        let mut sums = false;
        for j in i..i+25 {
            for k in i..i+25 {
/*
                println!(
                    "i = {}, j = {}, k = {}. target = {}, sum1 = {}, sum2 = {}.",
                    i, j, k, target, numbers[j], numbers[k]
                );
*/
                if (j != k) && (target == numbers[j] + numbers[k]) {
                    sums = true;
                }
            }
            if sums == true {
                break;
            }
        }
        if sums == false {
            println!("{}", target);
            break;
        }
    }
}
