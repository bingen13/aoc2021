use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let s = read_to_string("input.txt").unwrap();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut prod: Vec<(u32, u32)> = Vec::new();
    let s = s.split("do()");
    for i in s {
        if let Some(i) = i.split("don't()").next() {
            prod.extend(
                re.captures_iter(i)
                    .map(|caps| {
                        let (_, [m1, m2]) = caps.extract();
                        (m1.parse::<u32>().unwrap(), m2.parse::<u32>().unwrap())
                    })
                    .collect::<Vec<(u32, u32)>>(),
            );
        }
    }
    let r = prod.iter().fold(0, |acc, (m1, m2)| acc + (m1 * m2));
    println!("{}", r);
}
