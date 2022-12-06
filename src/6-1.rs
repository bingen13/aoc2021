use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut v = Vec::new();
    for i in f.chars() {
        v.push(i);
    }
    for i in 3..v.len() {
        let mut h = HashSet::new();
        h.insert(v[i - 3]);
        h.insert(v[i - 2]);
        h.insert(v[i - 1]);
        h.insert(v[i]);
        if h.len() == 4 {
            println!("{}", i + 1);
            break;
        }
    }
}
