use std::fs::read_to_string;

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

    let mut visible = 0;

    for (i, j) in trees.iter().enumerate() {
        if i < rows {
            visible += 1;
        } else if i >= length - rows {
            visible += 1;
        } else if i % rows == 0 {
            visible += 1;
        } else if i % rows == rows - 1 {
            visible += 1;
        } else if trees
            .iter()
            .enumerate()
            .filter(|(n, _)| (n < &i) & ((n % rows) == (i % rows)))
            .max_by(|m1, m2| m1.1.cmp(m2.1))
            .unwrap()
            .1
            < &j
        {
            visible += 1;
        } else if trees
            .iter()
            .enumerate()
            .filter(|(n, _)| (n > &i) & ((n % rows) == (i % rows)))
            .max_by(|m1, m2| m1.1.cmp(m2.1))
            .unwrap()
            .1
            < &j
        {
            visible += 1;
        } else if trees
            .iter()
            .enumerate()
            .filter(|(n, _)| (n >= &(rows * (i / rows))) & (n < &i))
            .max_by(|m1, m2| m1.1.cmp(m2.1))
            .unwrap()
            .1
            < &j
        {
            visible += 1;
        } else if trees
            .iter()
            .enumerate()
            .filter(|(n, _)| (n > &i) & (n < &(rows * ((i / rows) + 1))))
            .max_by(|m1, m2| m1.1.cmp(m2.1))
            .unwrap()
            .1
            < &j
        {
            visible += 1;
        }
    }

    println!("{}", visible);
}
