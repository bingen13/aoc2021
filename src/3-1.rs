use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::read_to_string;

fn main() {
    let mut truths = HashMap::new();
    let mut lies = HashMap::new();
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n");
    for value in f {
        if value.len() == 0 {
            break;
        }
        let l = value.len() - 1;
        for (j, bit) in value.chars().enumerate() {
            let truth = truths.entry(l - j).or_insert(0);
            let lie = lies.entry(l - j).or_insert(0);
            match bit {
                '0' => *lie += 1,
                '1' => *truth += 1,
                _ => println!("Error! {})", bit),
            }
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;

    println!("Truths: {:?}. Lies: {:?}.", &truths, &lies);
    for (k, v) in truths {
        if &v > lies.get(&k).unwrap() {
            {
                gamma += 2_u32.pow(k.try_into().unwrap());
            }
        } else {
            {
                epsilon += 2_u32.pow(k.try_into().unwrap());
            }
        }
    }
    println!(
        "Gamma: {}. Epsilon: {}. Product: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}
