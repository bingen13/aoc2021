use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut s = f.split("\n\n");
    let pats: Vec<_> = s.next().unwrap().split(", ").collect();
    let pats = "^(".to_owned() + &pats.join("|") + ")+$";
    let r = Regex::new(&pats).unwrap();
    let m = s
        .next()
        .unwrap()
        .split('\n')
        .filter(|s| r.is_match(s))
        .count();
println!("{}", m);
}
