use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::read_to_string;

fn neighbours(n: &usize, m: &[usize], l: &usize) -> usize {
    let mut c = Vec::new();
    // Left.
    if *n > 0 {
        c.push(n - 1);
    }
    // Right.
    if *n < ((l * l) - 1) as usize {
        c.push(n + 1);
    }
    // Up.
    if *n >= *l as usize {
        c.push(n - *l as usize);
    }
    // Down.
    if *n < ((l * l) - l) as usize {
        c.push(n + *l as usize);
    }
    c.into_iter().filter(|e| m.contains(e)).count()
}

fn is_neighbour(n1: &usize, n2: &usize, l: &usize) -> bool {
    let d = max(n1, n2) - min(n1, n2);
    (d == *l) || ((d == 1) && (min(n1, n2) % l != l - 1))
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut m = HashMap::new();
    let l = f64::sqrt(f.chars().filter(|c| c.is_alphabetic()).count() as f64 + 1.0) as usize;
    for (i, ch) in f.chars().filter(|c| c.is_alphabetic()).enumerate() {
        if ch.is_alphabetic() {
            let n = m.entry(ch).or_insert(vec![]);
            n.push(i);
        }
    }
    // println!("{:?}", m);
    let mut regions = Vec::new();
    for k in m.keys() {
        let mut v = m.get(k).unwrap().clone();
        while !v.is_empty() {
            let mut c = Vec::new();
            c.push(v.remove(v.len() - 1));
            let mut r = Vec::new();
            'inner: loop {
                if c.iter().map(|e| neighbours(e, &v, &l)).sum::<usize>() > 0 {
                    r.extend(c.clone());
                    c = v
                        .clone()
                        .into_iter()
                        .filter(|e| c.iter().any(|e2| is_neighbour(e, e2, &l)))
                        .collect();
                    v.retain(|e| !c.contains(e));
                } else {
                    v.retain(|e| !c.contains(e));
                    r.extend(c);
                    break 'inner;
                }
            }
            regions.push(r);
        }
    }
    let mut acc = 0;
    for r in regions {
        let mut t = 0;
        for e in &r {
            t += 4 - neighbours(e, &r, &l);
        }
        acc += t * r.len();
    }
    println!("{}", acc);
}
