use regex::Regex;
use std::fs::read_to_string;

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
    for r in robots.iter_mut() {
        r[0] += r[2]*100;
        r[1] += r[3]*100;
        r[0] = r[0].rem_euclid(width);
        r[1] = r[1].rem_euclid(height);
    }
    let mut ul = 0;
    let mut ur = 0;
    let mut dl = 0;
    let mut dr = 0;
    for r in robots {
        if (r[0] < width / 2) && (r[1] < height / 2) {
            ul += 1;
        }
        if (r[0] < width / 2) && (r[1] > height / 2) {
            ur += 1;
        }
        if (r[0] > width / 2) && (r[1] < height / 2) {
            dl += 1;
        }
        if (r[0] > width / 2) && (r[1] > height / 2) {
            dr += 1;
        }
    }
    {}
    println!("{}", ul * ur * dl * dr);
}
