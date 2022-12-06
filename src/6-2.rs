use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut v = Vec::new();
    for i in f.chars() {
        v.push(i);
    }
    for i in 13..v.len() {
        let mut h = HashSet::new();
        h.insert(v[i - 13]);
        h.insert(v[i - 12]);
        h.insert(v[i - 11]);
        h.insert(v[i - 10]);
        h.insert(v[i - 9]);
        h.insert(v[i - 8]);
        h.insert(v[i - 7]);
        h.insert(v[i - 6]);
        h.insert(v[i - 5]);
        h.insert(v[i - 4]);
        h.insert(v[i - 3]);
        h.insert(v[i - 2]);
        h.insert(v[i - 1]);
        h.insert(v[i]);
        if h.len() == 14 {
            println!("{}", i + 1);
            break;
        }
    }
}
