use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n");
    let mut numbers = Vec::new();
    let r = Regex::new(r"(\d*)").unwrap();
    let target = 756008079;
    for s in f {
        for i in r.captures_iter(s) {
            if let Ok(n) = i[1].parse::<u64>() {
                numbers.push(n);
            }
        }
    }
    for i in 0..numbers.len() {
        let mut sum = numbers[i];
        for j in i + 1..numbers.len() {
            if sum > target {
                break;
            }
            sum += numbers[j];
            if sum == target {
                println!(
                    "{}",
                    numbers[i..=j].iter().min().unwrap() + numbers[i..=j].iter().max().unwrap()
                );
                break;
            }
        }
    }
}
