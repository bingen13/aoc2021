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
    numbers.push(0);
    numbers.sort();
    println!("{:?}", numbers);
    let mut series = Vec::new();
    let mut i = 0;
    while i < numbers.len() - 1 {
        for j in (i + 1)..numbers.len() {
            if (j - i) as u64 != numbers[j] - numbers[i] {
                if j - i > 2 {
                    println!("{:?}", series);
                    series.push(j - i);
                    println!("{:?}", series);
                }
                i = j - 1;
                break;
            }
            if (j == numbers.len() - 1) && j - i > 2 {
                series.push(j - i + 1);
                i = j;
            }
        }
        i += 1;
    }
    println!("{:?}", series);
    let mut result: u64 = 1;
    for i in series.iter() {
        match i {
            3 => result *= 2,
            4 => result *= 4,
            5 => result *= 7,
            _ => println!("Error!"),
        }
    }
    println!("{}", result);
}
