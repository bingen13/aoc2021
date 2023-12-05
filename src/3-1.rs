use regex::Regex;
use std::fs::read_to_string;

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
    let mut total = 0;
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
            //            println!("{}: {:?}", number, sur);
            total += number;
        }
    }
    println!("{}", total);
}
