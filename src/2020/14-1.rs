use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n");
    let command = Regex::new(r"(mask|mem)(?:(?: = ((?:0|1|X)+))|(?:\[(\d+)\] = (\d+)))").unwrap();

    let mut inst = HashMap::new();
    let mut mask1: u64 = 0;
    let mut mask0: u64 = 0;
    let mut acc: u64 = 0;
    for i in f {
        if i.len() > 2 {
            for cap in command.captures_iter(i) {
                match &cap[1] {
                    "mask" => {
                        let mask_pattern = &cap[2];
                        mask0 = 0;
                        mask1 = 0;
                        for i in mask_pattern.chars() {
                            if (i == '0') | (i == '1') | (i == 'X') {
                                mask1 *= 2;
                                mask0 *= 2;
                            }
                            match i {
                                '0' => mask0 += 1,
                                '1' => mask1 += 1,
                                'X' => (),
                                _ => println!("Error! {}.", i),
                            }
                        }
                    }
                    "mem" => {
                        inst.insert(
                            cap[3].parse::<u64>().unwrap(),
                            (cap[4].parse::<u64>().unwrap() & !mask0) | mask1,
                        );
                    }
                    _ => (),
                }
            }
        }
    }
    for (_, i) in inst {
        acc += i;
    }
    println!("{}", acc);
}
