use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n");
    let mut values = HashSet::new();
    for (i, j) in f.enumerate() {
        if j.len() == 0 {
            break;
        }
        for (k, l) in j.chars().enumerate() {
            match l {
                '0'..='8' => {
                    values.insert((i, k));
                }
                '9' => (),
                _ => println!("Error."),
            }
        }
    }
    let mut basins = Vec::new();
    loop {
        if values.len() == 0 {
            break;
        }
        let mut b = HashSet::new();
        let val = values.iter().next().unwrap().clone();
        values.remove(&val);
        b.insert(val);
        while let Some(newval) = values.iter().find(|p| belongs(&b, p)).cloned() {
            values.remove(&newval);
            b.insert(newval);
        }
        basins.push(b);
    }
    let mut sizes = Vec::new();
    for i in basins.iter() {
        sizes.push(i.len());
    }
    sizes.sort();
    sizes.reverse();
    println!("{}.", &sizes[0] * &sizes[1] * &sizes[2]);
}

fn belongs(b: &HashSet<(usize, usize)>, p: &(usize, usize)) -> bool {
    if p.0 > 0 {
        if b.contains(&(p.0 - 1, p.1)) {
            return true;
        }
    }
    if p.1 > 0 {
        if b.contains(&(p.0, p.1 - 1)) {
            return true;
        }
    }
    return b.contains(&(p.0 + 1, p.1)) || b.contains(&(p.0, p.1 + 1));
}
