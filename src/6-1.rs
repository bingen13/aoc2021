use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut f = f.split('\n');
    let re = Regex::new(r"\d+").unwrap();
    let time = re
        .find_iter(f.next().unwrap())
        .map(|x| x.as_str().parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let distance = re
        .find_iter(f.next().unwrap())
        .map(|x| x.as_str().parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let l = time.len();
    let mut total = Vec::new();
    for i in 0..l {
        let mut j = 0;
        while (time[i] - j) * j <= distance[i] {
            j += 1;
        }
        total.push((time[i] + 1) - (j * 2));
    }
    println!("{}", total.iter().fold(1, |x, y| x * y));
}
