use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let l = f.split('\n').count() - 1;
    let mut m = Vec::new();
    let f = f.split('\n');
    for (i, line) in f.enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch.is_alphanumeric() {
                m.push((ch, (i, j)));
            }
        }
    }
    let h: HashSet<_> = m.iter().map(|e| e.0).collect();
    let mut anti = HashSet::new();
    'outer: for i in h {
        let mut v = m
            .iter()
            .filter_map(|e| if e.0 != i { None } else { Some(e.1) })
            .collect::<Vec<_>>();
        loop {
            let e = v.remove(0);
            if v.len() < 1 {
                continue 'outer;
            }
            for j in &v {
                if e.1 <= j.1 {
                    if (e.0 >= (j.0 - e.0)) && (e.1 >= (j.1 - e.1)) {
                        anti.insert((e.0 - (j.0 - e.0), e.1 - (j.1 - e.1)));
                    }
                    anti.insert((j.0 + (j.0 - e.0), j.1 + (j.1 - e.1)));
                } else {
                    if (e.0 >= (j.0 - e.0)) {
                        anti.insert((e.0 - (j.0 - e.0), e.1 + (e.1 - j.1)));
                    }
                    if (j.1 >= (e.1 - j.1)) {
                        anti.insert((j.0 + (j.0 - e.0), j.1 - (e.1 - j.1)));
                    }
                }
            }
        }
    }
    println!(
        "{}",
        anti.iter()
            .filter(|(x1, x2)| (*x1 < l) && (*x2 < l))
            .count()
    );
}
