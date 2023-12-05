use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let re = Regex::new(r"\d+").unwrap();

    let mut cards = HashMap::new();
    for i in 0..f.clone().count() - 1 {
        cards.insert(i, 1);
    }

    for (j, i) in f.enumerate() {
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
        let same = numbers1.intersection(&numbers2).count();
        let multiplier = cards.get(&j).unwrap().clone();
        for k in 0..same {
            if cards.contains_key(&(k + j + 1)) {
                cards.entry(&k + j + 1).and_modify(|c| *c += multiplier);
            }
        }
    }
/*
    for i in 0..cards.keys().count() {
        println!("{}: {}", i, cards.get(&i).unwrap());
    }
*/

    println!("{}", cards.values().sum::<usize>());
}
