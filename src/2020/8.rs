use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n");
    let reg = Regex::new(r"([[:alpha:]]{3}) (-|\+)(\d*)").unwrap();
    let mut mem = Vec::new();
    for s in f {
        for cap in reg.captures_iter(&s) {
            let cmd = cap[1].to_string();
            let param = cap[3].parse::<i32>().unwrap();
            if &cap[2] == "+" {
                mem.push((cmd, param));
            } else {
                mem.push((cmd, -1 * param));
            }
        }
    }

    for i in 0..mem.len() {
        if let Some(result) = run(&mem, i) {
            println!("{:?}", result)
        };
    }
}

fn run(v: &[(String, i32)], u: usize) -> Option<i32> {
    let mut ip: usize = 0;
    let mut acc: i32 = 0;
    let mut visited = HashSet::new();
    while ip < v.len() {
        let (i, j) = &v[ip];
        visited.insert(ip);
        match i.as_str() {
            "acc" => {
                acc += j;
                ip += 1;
            }
            "nop" => {
                if ip != u {
                    ip += 1;
                } else {
                    if visited.contains(&inst(ip, *j)) {
                        return None;
                    } else {
                        ip = inst(ip, *j);
                    }
                }
            }
            "jmp" => {
                if ip == u {
                    ip += 1;
                } else {
                    if visited.contains(&inst(ip, *j)) {
                        return None;
                    } else {
                        ip = inst(ip, *j);
                    }
                }
            }

            _ => {
                println!("Error! {}", i.to_string())
            }
        }
    }
    return Some(acc);
}

fn inst(a: usize, b: i32) -> usize {
    if b.is_negative() {
        a - b.wrapping_abs() as u32 as usize
    } else {
        a + b as usize
    }
}
