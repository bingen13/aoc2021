use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::read_to_string;

fn segment(points: &[usize]) -> Vec<(usize, usize)> {
    let mut started = false;
    let mut start = 0;
    let mut end = 0;
    let mut segments = Vec::new();
    for p in points.iter() {
        if !started {
            started = true;
            start = *p;
            end = *p;
        } else if end == *p - 1 {
            end = *p;
        } else {
            segments.push((start, end));
            start = *p;
            end = *p;
        }
    }
    if started {
        segments.push((start, end));
    }
    segments
}

fn neighbours(n: &usize, m: &[usize], l: &usize) -> usize {
    let mut c = Vec::new();
    // Left.
    if *n > 0 {
        c.push(n - 1);
    }
    // Right.
    if *n < ((l * l) - 1) {
        c.push(n + 1);
    }
    // Up.
    if *n >= *l {
        c.push(n - *l);
    }
    // Down.
    if *n < ((l * l) - l) {
        c.push(n + *l);
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
    let mut regions = Vec::new();
    for k in m.keys() {
        let mut v = m.get(k).unwrap().clone();
        while !v.is_empty() {
            let mut c = vec![v.remove(v.len() - 1)];
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
    for r in &regions {
        let mut sides = 0;
        let highest = r.iter().map(|e| e / l).min().unwrap();
        let lowest = r.iter().map(|e| e / l).max().unwrap();
        let left = r.iter().map(|e| e % l).min().unwrap();
        let right = r.iter().map(|e| e % l).max().unwrap();
        let mut h = Vec::new();
        for i in highest..=lowest {
            let mut row = Vec::new();
            for j in left..=right {
                if r.contains(&((i * l) + j)) {
                    row.push(j);
                }
            }
            row.sort();
            h.push(segment(&row));
        }
        // Upper sides.
        sides += h[0].len();
        for i in 1..h.len() {
            let mut p = Vec::new();
            for j in left..=right {
                if h[i].iter().any(|(e1, e2)| (*e1 <= j) && (*e2 >= j))
                    && !h[i - 1].iter().any(|(e1, e2)| (*e1 <= j) && (*e2 >= j))
                {
                    p.push(j);
                }
            }
            sides += segment(&p).len();
        }
        // Lower sides.
        sides += h.last().unwrap().len();
        for i in 1..h.len() {
            let mut p = Vec::new();
            for j in left..=right {
                if h[h.len() - i - 1]
                    .iter()
                    .any(|(e1, e2)| (*e1 <= j) && (*e2 >= j))
                    && !h[h.len() - i]
                        .iter()
                        .any(|(e1, e2)| (*e1 <= j) && (*e2 >= j))
                {
                    p.push(j);
                }
            }
            sides += segment(&p).len();
        }
        acc += sides * r.len();
    }
    println!("{}", acc * 2);
}
