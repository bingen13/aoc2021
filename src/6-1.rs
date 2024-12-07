use std::collections::HashSet;
use std::fs::read_to_string;

fn turn(d: (i32, i32)) -> (i32, i32) {
    match d {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => (0, 0),
    }
}

fn add(s1: &mut usize, s2: i32) {
    if s2 > 0 {
        *s1 += s2 as usize;
    } else if s2 < 0 {
        *s1 -= s2.abs() as usize;
    }
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let l = f.split('\n').count();
    let mut obstacles = HashSet::new();
    let mut start = (0, 0);
    let mut dir: (i32, i32) = (-1, 0);
    for (i, line) in f.split('\n').enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == '#' {
                obstacles.insert((i, j));
            } else if ch == '^' {
                start = (i, j);
            }
        }
    }
    let mut visits = HashSet::new();
    visits.insert(start);
    loop {
        let mut step = start;
        add(&mut step.0, dir.0);
        add(&mut step.1, dir.1);
        if obstacles.contains(&step) {
            dir = turn(dir);
            continue;
        }
        visits.insert(step);
        if (step.0 == 0) || (step.0 == l - 1) || (step.1 == 0) || (step.1 == l - 1) {
            break;
        }
        start = step;
    }
    println!("{}", visits.len() - 1);
}
