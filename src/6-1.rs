use std::collections::BTreeMap;
use std::fs::read_to_string;

fn main() {
    let mut f = read_to_string("input.txt").unwrap();
    f.retain(|c| !c.is_whitespace());
    let f = f.split(",");
    let mut fishes = Vec::new();
    for i in f {
        fishes.push(i.parse::<u64>().unwrap());
    }
    let mut fishmap: BTreeMap<u64, u64> = BTreeMap::new();
    for i in fishes.iter() {
        *fishmap.entry(*i).or_insert(0) += 1;
    }
    for _ in 0..80 {
        let sum: u64 = fishmap.values().sum();
        println!("{}", sum);
        fishmap = day(fishmap);
    }
    let sum: u64 = fishmap.values().sum();
    println!("{}", sum);
}

fn day(f: BTreeMap<u64, u64>) -> BTreeMap<u64, u64> {
    let mut g = BTreeMap::new();

    for (i, j) in f {
        if i == 0 {
            g.insert(6, j);
            g.insert(8, j);
        } else {
            *g.entry(i - 1).or_insert(0) += j;
        }
    }
g
}
