use regex::Regex;
use std::cmp::{max, min};
use std::fs::read_to_string;

fn comdif(a: usize, b: usize) -> usize {
    max(a, b) - min(a, b)
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let f = f
        .filter_map(|s| if s == "" { None } else { Some(s.to_string()) })
        .collect::<Vec<String>>();

    let re = Regex::new(r"(\d+)").unwrap();
    let mut numbers = Vec::new();
    for (i, j) in f.iter().enumerate() {
        for k in re.captures_iter(j) {
            numbers.push((
                i,
                k.get(1)
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse::<u32>()
                    .unwrap(),
                k.get(1).unwrap().start(),
                k.get(1).unwrap().end(),
            ));
        }
    }
    let l = f[0].len();
    let mut ranges = Vec::new();

    for n in numbers {
        let (line, number, start, end) = n;
        let mut sur = Vec::new();
        let min = if start == 0 { 0 } else { start - 1 };
        let max = if end == l { l } else { end + 1 };
        if line != 0 {
            for i in f[line - 1][min..max].chars() {
                sur.push(i);
            }
        }
        if line != l - 1 {
            for i in f[line + 1][min..max].chars() {
                sur.push(i);
            }
        }
        if start != 0 {
            sur.push(f[line][min..].chars().next().unwrap());
        }
        if end != l {
            sur.push(f[line][..end + 1].chars().rev().next().unwrap());
        }
        if sur.iter().any(|&c| c != '.') {
            ranges.push((line, start, end - 1, number));
        }
    }
    let mut total = 0;
    for (i, j) in f.iter().enumerate() {
        for (k, l) in j.chars().enumerate() {
            if l == '*' {
                let result = ranges
                    .iter()
                    .filter(|f| {
                        comdif(f.0, i) <= 1
                            && ((f.1..=f.2).contains(&k) || (k == f.2 + 1) || k == f.1 - 1)
                    })
                    .map(|x| x.3)
                    .collect::<Vec<u32>>();
                if result.len() == 2 {
                    total += result[0] * result[1];
                }
            }
        }
    }
    println!("{}", total);
}
