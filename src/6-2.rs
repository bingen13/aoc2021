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
    let l = f.split('\n').count() - 1;
    let mut obstacles = HashSet::new();
    let mut start = (0, 0);
    for (i, line) in f.split('\n').enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == '#' {
                obstacles.insert((i, j));
            } else if ch == '^' {
                start = (i, j);
            }
        }
    }
    let truestart = start;
    let mut acc = 0;
    for i in 0..l {
        for j in 0..l {
            if obstacles.contains(&(i, j)) {
                continue;
            }
            let mut found = false;
            let mut visits = HashSet::new();
            let mut dir: (i32, i32) = (-1, 0);
            let mut start = truestart;
            obstacles.insert((i, j));
            loop {
                if visits.contains(&(start, dir)) {
                    found = true;
                    break;
                }
                visits.insert((start, dir));
                let mut step = start;
                add(&mut step.0, dir.0);
                add(&mut step.1, dir.1);
                if obstacles.contains(&step) {
                    dir = turn(dir);
                    continue;
                }
                if (step.0 == 0) || (step.0 == l - 1) || (step.1 == 0) || (step.1 == l - 1) {
                    break;
                }
                start = step;
            }
            if found == true {
                acc += 1;
            }
            obstacles.remove(&(i, j));
        }
    }
    println!("{}", acc);
}
