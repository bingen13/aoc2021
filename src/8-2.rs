use std::collections::HashSet;
use std::fs::read_to_string;

fn slope(p1: &(i32, i32), p2: &(i32, i32)) -> f64 {
    (p1.0 - p2.0) as f64 / (p1.1 - p2.1) as f64
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let l = f.split('\n').count() - 1;
    let mut m = Vec::new();
    let f = f.split('\n');
    for (i, line) in f.enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch.is_alphanumeric() {
                m.push((ch, (i as i32, j as i32)));
            }
        }
    }
    let h: HashSet<_> = m.iter().map(|e| e.0).collect();
    let mut anti: HashSet<_> = m.iter().map(|e| e.1).collect();
    for i in 0..l {
        'outer: for j in 0..l {
            'inner: for k in &h {
                let mut points: Vec<_> = m
                    .iter()
                    .filter_map(|e| if e.0 == *k { Some(e.1) } else { None })
                    .collect();
                loop {
                    let p = points[0];
                    points.remove(0);
                    if points.len() < 1 {
                        continue 'inner;
                    }
                    for p2 in &points {
                        if slope(&p, &(i as i32, j as i32)) == slope(&p, p2) {
                            anti.insert((i as i32, j as i32));
                            continue 'outer;
                        }
                    }
                }
            }
        }
    }
    println!("{}", anti.len());
}
