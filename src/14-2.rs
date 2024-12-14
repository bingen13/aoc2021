use regex::Regex;
use std::fs::read_to_string;

fn segment(points: &[i64]) -> i64 {
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
    segments.iter().map(|e| e.1 - e.0).max().unwrap()
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let re = Regex::new(r"-?\d+").unwrap();
    let mut robots = Vec::new();
    for i in f.split('\n') {
        if i.is_empty() {
            break;
        }
        robots.push(
            re.find_iter(i)
                .filter_map(|e| e.as_str().parse::<i64>().ok())
                .collect::<Vec<i64>>(),
        );
    }
    let width = 101;
    let height = 103;
    let mut top = 0;
    let mut i1 = 0;
    for i in 1..=(height * width) {
        let mut rb: Vec<_> = robots.iter().map(|r| (r[0], r[1])).collect();
        let rbs = rb.len();
        rb.sort();
        rb.dedup();
        if rb.len() == rbs {
            println!("Loop {}.", i-1 );
        }
        for r in robots.iter_mut() {
            r[0] += r[2];
            r[1] += r[3];
            r[0] = r[0].rem_euclid(width);
            r[1] = r[1].rem_euclid(height);
        }
    }
    println!("{}", i1);
}
