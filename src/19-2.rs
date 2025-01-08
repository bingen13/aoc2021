use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut s = f.split("\n\n");
    let pat: Vec<_> = s.next().unwrap().split(", ").collect();
    let pats = "^(".to_owned() + &pat.join("|") + ")+$";
    let r = Regex::new(&pats).unwrap();
    let mut c = 0;
    let pat: HashSet<_> = pat.iter().collect();
    for i in s.next().unwrap().split('\n').filter(|s| r.is_match(s)) {
        let mut perms = vec![0_64; i.len() + 1];
        perms[0] = 1;
        for j in 0..perms.len() {
            if perms[j] != 0 {
                for k in j + 1..=i.len() {
                    if pat.contains(&(&i[j..k])) {
                        perms[k] += perms[j];
                    }
                }
            }
        }
        c += perms.last().unwrap();
    }
    println!("{}", c);
}
