use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n");
    let mut edges = Vec::new();
    for i in f {
        if i.len() == 0 {
            break;
        }
        let mut i = i.split("-");
        let origin = i.next().unwrap();
        let target = i.next().unwrap();
        let mut h = HashSet::new();
        h.insert(target.to_string());
        h.insert(origin.to_string());
        edges.push(h);
    }
    let s = "start".to_string();
    let mut small = HashMap::new();
    g
}

fn exits(v: &[HashSet<String>], s: &String) -> bool {
    v.iter().any(|i| i.contains(s))
}
