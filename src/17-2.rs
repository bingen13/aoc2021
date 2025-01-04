use regex::Regex;
use std::fs::read_to_string;

fn combo(op: u128, r1: u128, r2: u128, r3: u128) -> u128 {
    match op {
        0..=3 => op,
        4 => r1,
        5 => r2,
        6 => r3,
        _ => todo!(),
    }
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let re = Regex::new(r"\d+").unwrap();
    let mut s = f.split("\n\n");
    let s1 = s.next().unwrap();
    let s2 = s.next().unwrap();
    let [r1, r2, r3, ..]: [u128] = re
        .find_iter(s1)
        .map(|m| m.as_str().parse::<u128>().unwrap())
        .collect::<Vec<_>>()[..]
    else {
        todo!()
    };
    let inst: Vec<_> = re
        .find_iter(s2)
        .map(|m| m.as_str().parse::<u128>().unwrap())
        .collect();
    let mut output = Vec::new();
    let mut i = 0;
    loop {
        let mut r1 = i;
        let mut r2 = r2;
        let mut r3 = r3;
        let mut pointer = 0;
        output = Vec::new();
        while pointer < inst.len() {
            //println!("Pointer: {}. {}, {}, {}.", pointer, r1, r2, r3);
            match inst[pointer] {
                0 => r1 /= 2_u32.pow(combo(inst[pointer + 1], r1, r2, r3) as u32) as u128,
                1 => r2 = r2 ^ inst[pointer + 1],
                2 => r2 = combo(inst[pointer + 1], r1, r2, r3) % 8,
                3 => {
                    if r1 != 0 {
                        pointer = inst[pointer + 1] as usize;
                        continue;
                    }
                }
                4 => r2 = r2 ^ r3,
                5 => output.push(combo(inst[pointer + 1], r1, r2, r3) % 8),
                6 => r2 = r1 / 2_u32.pow(combo(inst[pointer + 1], r1, r2, r3) as u32) as u128,
                7 => r3 = r1 / 2_u32.pow(combo(inst[pointer + 1], r1, r2, r3) as u32) as u128,
                _ => todo!(),
            }
            pointer += 2;
        }
        if output
            .iter()
            .rev()
            .zip(inst.iter().rev())
            .all(|(a, b)| a == b)
        {
            println!("Level: {}. {:b}", output.len(), i);
            if output.len() == inst.len() {
                println!("Eureka: {}", i);
                break;
            }

            i *= 8;
        }
        i += 1;
    }
}
