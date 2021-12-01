use std::fs::read_to_string;

fn main() {
    let mut passes = Vec::new();
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n");
    for i in f {
        let mut s = String::new();
        for j in i.chars() {
            s.push(match j {
                'B' | 'R' => '1',
                _ => '0',
            });
        }
        if s.len() > 0 {
            passes.push(u32::from_str_radix(&s, 2).unwrap());
        }
    }

    passes.sort();
    for i in *passes.iter().min().unwrap()..*passes.iter().max().unwrap() {
        if !passes.contains(&i) {
            println!("Value: {}", i);
            break;
        }
    }

    println!("Max: {}", passes.iter().max().unwrap());
}
