use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let mut equations = Vec::new();
    for i in f {
        if i.is_empty() {
            break;
        }
        let mut i = i.split(": ");
        let r = i.next().unwrap().parse::<u64>().unwrap();
        let ops = i
            .next()
            .unwrap()
            .split(' ')
            .map(|m| m.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        equations.push((r, ops));
    }
    let mut acc: u64 = 0;
    'outer: for (key, val) in &equations {
        for i in 0..(3_u64.pow(val.len() as u32 - 1)) {
            let mut v = val[0];
            for j in 1..val.len() {
                match (i / (3_u64.pow(j as u32 - 1)) % 3) {
                    0 => v += val[j],
                    1 => v *= val[j],
                    2 => {
                        v = (v.to_string() + &val[j].to_string())
                            .parse::<u64>()
                            .unwrap()
                    }
                    _ => todo!(),
                }
            }
            if v == *key {
                acc += key;
                continue 'outer;
            }
        }
    }
    println!("{}", acc);
}
