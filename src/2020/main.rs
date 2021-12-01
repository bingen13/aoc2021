use regex::Regex;
use std::convert::TryInto;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Mask {
    bits: u64,
    floating: Vec<u64>,
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n");
    let command = Regex::new(r"(mask|mem)(?:(?: = ((?:0|1|X)+))|(?:\[(\d+)\] = (\d+)))").unwrap();

    let mut inst = Vec::new();
    let mut mask = get_mask("111111111111111111111111111111111111");
    let mut acc: u64 = 0;
    for i in f {
        if i.len() > 2 {
            for cap in command.captures_iter(i) {
                match &cap[1] {
                    "mask" => mask = get_mask(&cap[2]),
                    "mem" => inst.push((
                        mask.clone(),
                        cap[3].parse::<u64>().unwrap(),
                        cap[4].parse::<u64>().unwrap(),
                    )),
                    _ => (),
                }
            }
        }
    }
}

fn get_mask(s: &str) -> Mask {
    let mut b: u64 = 0;
    let mut v = Vec::new();
    for (i, j) in s.chars().enumerate() {
        match j {
            '1' => b += 2_u64.pow((36 - (i + 1)).try_into().unwrap()),
            'X' => v.push(2_u64.pow((36 - (i + 1)).try_into().unwrap())),
            _ => (),
        }
    }
    Mask {
        bits: b,
        floating: v,
    }
}
