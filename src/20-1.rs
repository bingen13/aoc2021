use std::cmp::{max, min};
use std::fs::read_to_string;

fn order(n: &[(i32, usize)]) {
    for i in 0..n.len() {
        print!("{} ", n.iter().find(|p| p.1 == i).unwrap().0);
    }
    println!();
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let mut numbers = Vec::new();
    for (i, j) in f.enumerate() {
        if j.is_empty() {
            break;
        }
        numbers.push((j.parse::<i32>().unwrap(), i));
    }
    let l = numbers.len();
    order(&numbers);
    for i in 0..numbers.len() {
        let n = numbers[i];
        let newplace = (n.0 + n.1 as i32).rem_euclid(l as i32);
        for j in 0..(n.0.abs()) % (l as i32) {
            let p = numbers
                .iter()
                .position(|x| x.1 == (n.1 + j as usize + 1) % l)
                .unwrap();
//            println!("{}, {}, {}", j, p, numbers[p].0);
            numbers[p].1 = (n.1 + j as usize) % l;
        }
        if n.0 < 0 {
            numbers[i].1 = (newplace - 1).rem_euclid(l as i32) as usize;
        } else {
            numbers[i].1 = newplace as usize;
        }
        println!("{:?}", numbers);
        order(&numbers);
    }
}
