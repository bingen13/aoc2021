use std::convert::TryInto;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut f = f.split("\n");
    let l = f.next().unwrap();

    let values = f.next().unwrap().split(",");
    let mut numbers = Vec::new();
    for i in values {
        if let Ok(x) = i.parse::<u64>() {
            numbers.push(x);
        } else {
            numbers.push(0);
        }
    }
    println!("{:?}", numbers);
    let mut v = Vec::new();
    for i in 0..numbers.len() {
        if numbers[i] != 0 {
            v.push((numbers[i], i));
        }
    }
    v.sort();
    v.reverse();
    println!("{:?}", v);

    let mut base: u64 = 1;
    let mut step = 1;
    let (mut num, mut offset) = v.pop().unwrap();
    let startnum = num;
    let startoffset = offset as u64;
    while v.len() > 0 {
        println!("Base: {}. Step: {}", base, step);
        let (a, b) = v.pop().unwrap();
        for i in (base..).step_by(step) {
            if ((startnum * i as u64) + b as u64 - startoffset) % a == 0 {
                base = i;
                break;
            }
        }
        step = (step as u64 * a as u64).try_into().unwrap();
        num = a;
        offset = b;
    }
    println!(
        "Base: {}. Result: {}",
        base,
        (base * startnum) - startoffset
    );
}
