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
        if let Some(n) = nodes.get_mut(&origin.to_string()) {
            n.push(i.next().unwrap().to_string());
        } else {
            nodes.insert(origin.to_string(), vec![i.next().unwrap().to_string()]);
        }
    }
}

fn exits(n: &HashMap<String, Vec<String>>, p: &[usize]) -> Option<Vec<String>> {
    let mut s = "start".to_string();
    for i in p.iter() {
        s = n.get(&s).unwrap()[*i].clone();
    }
    return n.get(&s).cloned();
}
