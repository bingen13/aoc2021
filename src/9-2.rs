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
        while values.iter().any(|p| belongs(&b, p)) {
            let newval = values.iter().find(|p| belongs(&b, p)).unwrap().clone();
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

fn neighbour(a: &(usize, usize), b: &(usize, usize)) -> bool {
    if a.0 == b.0 {
        (a.1 < b.1 && a.1 + 1 == b.1) || (a.1 > b.1 && a.1 - 1 == b.1)
    } else if a.1 == b.1 {
        (a.0 < b.0 && a.0 + 1 == b.0) || (a.0 > b.0 && a.0 - 1 == b.0)
    } else {
        false
    }
}

fn belongs(b: &HashSet<(usize, usize)>, p: &(usize, usize)) -> bool {
    for i in b.iter() {
        if neighbour(i, p) {
            return true;
        }
    }
    return false;
}
