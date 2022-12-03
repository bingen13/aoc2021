use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let mut priors = 0;
    for i in f {
        let mut h1 = HashSet::new();
        let mut h2 = HashSet::new();
        let l = i.len();
        if l > 0 {
            for (j, k) in i.chars().enumerate() {
                if j < l / 2 {
                    h1.insert(k);
                } else {
                    h2.insert(k);
                }
            }
        }
        let common = h1.intersection(&h2);
        for m in common {
            if m.is_uppercase() {
                priors += 27;
            } else {
                priors += 1;
            }
            priors += m.to_lowercase().next().unwrap() as u32 - 'a' as u32;
        }
    }
    println!("{}", priors);
}
