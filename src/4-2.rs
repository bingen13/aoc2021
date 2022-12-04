use std::cmp::{max, min};
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let mut pairs = 0;
    for i in f {
        if i.len() > 0 {
            let mut j = i.split(',');
            let p1 = j.next().unwrap();
            let p2 = j.next().unwrap();
            let mut p1 = p1.split('-');
            let n1 = p1.next().unwrap().parse::<u32>().unwrap();
            let n2 = p1.next().unwrap().parse::<u32>().unwrap();
            let mut p2 = p2.split('-');
            let n3 = p2.next().unwrap().parse::<u32>().unwrap();
            let n4 = p2.next().unwrap().parse::<u32>().unwrap();
            if max(n1, n3) <= min(n2, n4) {
                pairs += 1;
            }
        }
    }
    println!("{}", pairs);
}
