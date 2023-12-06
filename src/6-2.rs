use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut f = f.split('\n');
    let re = Regex::new(r"\d+").unwrap();
    let time = re
        .find_iter(f.next().unwrap())
        .map(|x| x.as_str())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = re
        .find_iter(f.next().unwrap())
        .map(|x| x.as_str())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let mut j = 0;
    while (time - j) * j <= distance {
        j += 1;
    }
    let total = (time + 1) - (j * 2);
    println!("{}", total);
}
