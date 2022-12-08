use std::cmp::max;
use std::fs::read_to_string;

fn up(i: usize, l: usize) -> bool {
    i >= l
}

fn down(i: usize, l: usize) -> bool {
    i < l * (l - 1)
}

fn left(i: usize, l: usize) -> bool {
    i % l != 0
}

fn right(i: usize, l: usize) -> bool {
    i % l != (l - 1)
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let mut trees = Vec::new();
    let mut lines = 0;
    for (i, j) in f.enumerate() {
        if j.is_empty() {
            break;
        }
        lines = i;
        for c in j.chars() {
            trees.push(c.to_digit(10).unwrap());
        }
    }
    lines += 1;
    let length = trees.len();
    let rows = length / lines;

    let mut max_val = 0;
    for (i, j) in trees.iter().enumerate() {
        let mut ups = 0;
        let mut downs = 0;
        let mut lefts = 0;
        let mut rights = 0;
        let mut t = i;
        let mut value = &0;
        while up(t, rows) & (value < j) {
            ups += 1;
            t -= rows;
            value = &trees[t];
        }
        value = &0;
        t = i;
        while down(t, rows) & (value < j) {
            downs += 1;
            t += rows;
            value = &trees[t];
        }
        value = &0;
        t = i;
        while left(t, rows) & (value < j) {
            lefts += 1;
            t -= 1;
            value = &trees[t];
        }
        value = &0;
        t = i;
        while right(t, rows) & (value < j) {
            rights += 1;
            t += 1;

            value = &trees[t];
        }

        max_val = max(max_val, ups * downs * lefts * rights);
    }
    println!("{}", max_val);
}
