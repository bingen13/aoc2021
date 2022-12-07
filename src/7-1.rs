use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let mut cwd: Vec<(String, u64, Vec<String>)> = Vec::new();
    let mut dirs: HashMap<String, (u64, Vec<String>)> = HashMap::new();
    for i in f {
        let v: Vec<_> = i.split(' ').collect();

        // termination condition.
        if v[0].is_empty() {
            let elem = cwd.pop().unwrap();
            dirs.insert(elem.0, (elem.1, elem.2));
            break;
        }
        // Command.
        if v[0] == "$" {
            if v[1] == "cd" {
                if v[2] != ".." {
                    if !cwd.is_empty() {
                        let mut last = cwd.pop().unwrap();
                        let prev = last.0.clone() + &"/".to_string();
                        last.2.push(prev.clone() + &v[2].to_string());
                        cwd.push(last);
                        cwd.push((prev + &v[2], 0, Vec::new()));
                    } else {
                        cwd.push((v[2].to_string(), 0, Vec::new()));
                    }
                } else {
                    let elem = cwd.pop().unwrap();
                    dirs.insert(elem.0, (elem.1, elem.2));
                }
            }
        } else if v[0] != "dir" {
            let n = v[0].parse::<u64>().unwrap();
            let mut last = cwd.pop().unwrap();
            last.1 += n;
            cwd.push(last);
        }
    }
    let mut acc = 0;
    for (_, j) in &dirs {
        let mut s = j.0;
        let mut v = j.1.clone();
        while !v.is_empty() {
            let e = v.pop().unwrap();
            let k2 = dirs.get(&e).unwrap();
            s += k2.0;
            v.append(&mut k2.1.clone());
        }
        if s < 100000 {
            acc += s;
        }
    }
    println!("{}", acc);
}
