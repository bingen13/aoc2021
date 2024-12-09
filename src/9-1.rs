use std::cmp::min;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut v = Vec::new();
    for (i, ch) in f.chars().enumerate() {
        if !ch.is_digit(10) {
            break;
        }
        if i % 2 == 0 {
            v.push((Some(i / 2), ch.to_digit(10).unwrap()));
        } else {
            v.push((None, ch.to_digit(10).unwrap()));
        }
    }
    while v.iter().any(|e| e.0 == None) {
        let left = v.iter().position(|e| e.0 == None).unwrap();
        let right = v.len() - 1;
        let n = min(v[left].1, v[right].1);
        let val = v[right].0;
        if v[right].1 == n {
            v.remove(right);
        } else {
            v[right].1 -= n;
        }
        if v[left].1 == n {
            v[left] = (val, n);
        } else {
            v[left].1 -= n;
            v.insert(left, (val, n));
        }
        if v.last().unwrap().0 == None {
            v.remove(v.len() - 1);
        }
    }
    while let Some((i, e)) = v.windows(2).enumerate().find(|(i, e)| e[0].0 == e[1].0) {
        v[i] = (v[i].0, v[i].1 + v[i + 1].1);
        v.remove(i + 1);
    }
    let mut s = 0;
    let mut i = 0;
    for val in v {
        for j in 0..val.1 {
            s += (i + j) as usize * val.0.unwrap();
        }
        i += val.1;
    }
    println!("{}", s);
}
