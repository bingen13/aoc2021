use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let mut priors = 0;
    let mut h1 = HashSet::new();
    let mut h2 = HashSet::new();
    let mut h3 = HashSet::new();

    for (i, j) in f.enumerate() {
        if i % 3 == 0 {
            h1 = HashSet::new();
            h2 = HashSet::new();
            h3 = HashSet::new();
        }
        for k in j.chars() {
            match i % 3 {
                0 => {
                    h1.insert(k);
                }
                1 => {
                    h2.insert(k);
                }
                2 => {
                    h3.insert(k);
                }
                _ => (),
            }
        }
        if i % 3 == 2 {
            let common = h1.iter()
                .filter(|n| h2.contains(&n) & h3.contains(&n))
                .next()
                .unwrap();
            priors += 1;
            if common.is_uppercase() {
                priors += 26;
            }
            priors += common.to_lowercase().next().unwrap() as u32 - 'a' as u32;
        }
    }
println!("{}", priors);
}
