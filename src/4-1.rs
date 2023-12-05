use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let re = Regex::new(r"\d+").unwrap();

    let mut total = 0;
    for i in f {
        if i.is_empty() {
            break;
        }
        let mut n = i.split(": ").nth(1).unwrap().split('|');
        let numbers1 = re
            .find_iter(n.next().unwrap())
            .map(|x| x.as_str().parse::<u32>().unwrap())
            .collect::<HashSet<_>>();
        let numbers2 = re
            .find_iter(n.next().unwrap())
            .map(|x| x.as_str().parse::<u32>().unwrap())
            .collect::<HashSet<_>>();
        let same= numbers1.intersection(&numbers2).count();
        if same > 0 {
            total += 2_u32.pow(same as u32 - 1);
        }
    }
    println!("{}", total);
}
