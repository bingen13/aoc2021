use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n");
    let mut nodes: HashMap<String, Vec<String>> = HashMap::new();
    for i in f {
        if i.len() == 0 {
            break;
        }
        let mut i = i.split("-");
        let origin = i.next().unwrap();
        let target = i.next().unwrap();
        if let Some(n) = nodes.get_mut(&origin.to_string()) {
            n.push(target.to_string());
        } else {
            nodes.insert(origin.to_string(), vec![target.to_string()]);
        }
        if let Some(n) = nodes.get_mut(&target.to_string()) {
            n.push(origin.to_string());
        } else {
            nodes.insert(target.to_string(), vec![origin.to_string()]);
        }
    }
    let mut paths = 0;
    let s = "start".to_string();
    let mut visits = Vec::new();
    visits.push(s.clone());
    let mut small = HashSet::new();
    small.insert("start".to_string());
    let mut candidates: Vec<usize> = Vec::new();
    candidates.push(0);
    candidates.push(0);
    while visits.len() > 0 {
        let c = candidates.last().unwrap().clone();
        if let Some(i) = nodes.get(visits.last().unwrap()) {
            if i.len() > c {
                let j = &i[c];
                if j == &"end".to_string() {
                    paths += 1;
                    candidates.pop();
                    candidates.push(c + 1);
                    continue;
                } else {
                    if !small.contains(j) {
                        visits.push(j.clone());
                        candidates.push(0);
                        if j.chars().next().unwrap().is_lowercase() {
                            small.insert(j.clone());
                        }
                        continue;
                    } else {
                        candidates.pop();
                        candidates.push(c + 1);
                        continue;
                    }
                }
            } else {
                let e = visits.pop().unwrap();
                if small.contains(&e) {
                    small.remove(&e);
                }
                candidates.pop();
                let d = candidates.pop().unwrap();
                candidates.push(d + 1);
                continue;
            }
        } else {
            let e = visits.pop().unwrap();
            if small.contains(&e) {
                small.remove(&e);
            }
            candidates.pop();
            let d = candidates.pop().unwrap();
            candidates.push(d + 1);
        }
    }
    println!("{}", paths);
}
