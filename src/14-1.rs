use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut f = f.split("\n\n");
    let mut start = f.next().unwrap().to_string().chars().collect::<Vec<char>>();
    let mut transforms = HashMap::new();
    for i in f.next().unwrap().split("\n") {
        if i.len() == 0 {
            break;
        }
        let mut j = i.split(" -> ");
        let origin = j.next().unwrap().to_string();
        let target = j.next().unwrap().to_string();
        transforms.insert(origin, target);
    }
    for _ in 0..10 {
        let mut result = Vec::new();
        result.push(*start.first().unwrap());
        for i in start[..].windows(2) {
            let c1 = i[0];
            let c2 = i[1];
            let mut s = String::new();
            s.push(c1);
            s.push(c2);
            if let Some(inter) = transforms.get(&s) {
                result.push(inter.chars().next().unwrap());
            }
            result.push(c2);
        }
        start = result.clone();
    }
    let mut h: Vec<(char, u64)> = Vec::new();
    for i in start.iter() {
        if let Some(j) = h.iter().position(|x| x.0 == *i) {
            h[j].1 += 1;
        } else {
            h.push((*i, 1));
        }
    }
    let m1 = h
        .iter()
        .map(|x| x.1)
        .reduce(|x, y| if x > y { x } else { y })
        .unwrap();
    let m2 = h
        .iter()
        .map(|x| x.1)
        .reduce(|x, y| if x < y { x } else { y })
        .unwrap();

    println!("{}, {}, {}.", m1, m2, m1 - m2);
}
