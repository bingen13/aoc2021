use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let l = f.split('\n').count() - 1;
    let mut v = Vec::new();
    for i in f.chars() {
        if i.is_digit(10) {
            v.push(i.to_digit(10).unwrap());
        }
    }
    let mut zeros = v
        .iter()
        .enumerate()
        .filter_map(|(i, e)| if *e == 0 { Some(i) } else { None })
        .collect::<Vec<_>>();
    let mut acc = 0;
    while !zeros.is_empty() {
        let mut pos = Vec::new();
        pos.push(zeros.remove(zeros.len() - 1));
        for i in 1..=9 {
            let mut n = Vec::new();
            for p in pos {
                // Left.
                if (p % l != 0) && v[p - 1] == i {
                    n.push(p - 1);
                }
                // Right.
                if (p % l != l - 1) && v[p + 1] == i {
                    n.push(p + 1);
                }
                // Up.
                if (p >= l) && (v[p - l] == i) {
                    n.push(p - l);
                }
                // Down.
                if (p + l < v.len()) && (v[p + l] == i) {
                    n.push(p + l);
                }
            }
            pos = n;
        }
        acc += pos.len();
    }
    println!("{}", acc);
}
