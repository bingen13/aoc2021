use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let s = f.split("\n\n");
    let [s1, s2, ..] = s.collect::<Vec<_>>()[..] else {
        println!("Incorrect format.");
        return;
    };
    let rules: Vec<_> = s1.split('\n').filter(|s| !s.is_empty()).collect();
    let patterns: Vec<_> = s2.split('\n').filter(|s| !s.is_empty()).collect();
    let mut constraints = Vec::new();
    for i in rules {
        let mut i = i.split('|');
        let i1 = i.next().unwrap().parse::<u32>().unwrap();
        let i2 = i.next().unwrap().parse::<u32>().unwrap();
        constraints.push((i1, i2));
    }
    let mut acc = 0;
    let mut pat: Vec<Vec<_>> = Vec::new();
    for p in patterns {
        pat.push(p.split(',').map(|s| s.parse::<u32>().unwrap()).collect());
    }
    'pats: for p in 0..pat.len() {
        for c in &constraints {
            let (c1, c2) = (c.0, c.1);
            if let (Some(p1), Some(p2)) = (
                pat[p].iter().position(|e| *e == c1),
                pat[p].iter().position(|e| *e == c2),
            ) {
                if p1 >= p2 {
                    continue 'pats;
                }
            }
        }
        acc += pat[p][pat[p].len() / 2];
    }
    println!("{}", acc);
}
