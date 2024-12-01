use std::cmp::{max, min};
use std::fs::read_to_string;

fn main() {
    let mut l1 = Vec::<u32>::new();
    let mut l2 = Vec::<u32>::new();
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    for i in f {
        if i.is_empty() {
            break;
        }
        let mut s = i.split_whitespace().map(|s| s.parse::<u32>().unwrap());
        l1.push(s.next().unwrap());
        l2.push(s.next().unwrap());
    }
    l1.sort();
    l2.sort();
    let mut dist = 0;
    for i in 0..l1.len() {
        let n1 = l1[i];
        let n2 = l2[i];
        dist += max(n1, n2) - min(n1, n2);
    }
    println!("{}", dist);
}
