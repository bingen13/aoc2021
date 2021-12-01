use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n\n");
    let mut count = 0;
    for i in f {
        let mut members = 0;
        for j in i.split("\n") {
            if j != "" {
                members += 1
            }
        }
        let answer = occurrences(i);
        for (_, j) in &answer {
            if j == &members {
                count += 1;
            }
        }
    }

    println!("Count: {}", count);
}

fn occurrences(s: &str) -> HashMap<char, u32> {
    let mut map = HashMap::new();
    for i in s.chars() {
        if i != '\n' {
            if map.contains_key(&i) {
                map.insert(i, map[&i] + 1);
            } else {
                map.insert(i, 1);
            }
        }
    }
    return map;
}

