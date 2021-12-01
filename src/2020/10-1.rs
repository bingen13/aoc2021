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
    numbers.sort();
    let mut one = 1;
    let mut three = 1;
println!("{:?}", numbers);
    for i in 1..numbers.len() {
        if numbers[i] - numbers[i - 1] == 1 {
            one += 1;
        } else if numbers[i] - numbers[i - 1] == 3 {
            three += 1;
        }
    }
    println!("One: {}. Three: {}. Product: {}", one, three, one * three);


}
