use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    let re =
        Regex::new(r"Valve ([[:alpha:]]{2}).*rate=(\d+).*to valve(?:s)? ([A-Z(?:, )?]+)").unwrap();
    let re_valve = Regex::new(r"([A-Z]{2})").unwrap();
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let mut valves = Vec::new();
    let mut map = HashMap::new();
    for i in f {
        if let Some(cap) = re.captures(i) {
            let valve = cap[1].to_string();
            let flow = cap[2].parse::<u32>().unwrap();
            valves.push((valve.clone(), flow));
            let dest = &cap[3];
            let mut v = Vec::new();
            for cap2 in re_valve.captures_iter(dest) {
                v.push((cap2[1].to_string(), 1));
            }
            map.insert(valve, v);
        }
    }
    let mut valve_map = Vec::new();
    for i in 0..valves.len() {
        let s = valves[i].0.clone();
        let f = valves[i].1;
        let v: Vec<_> = map
            .get(&s)
            .unwrap()
            .iter()
            .map(|x| (valves.iter().position(|p| p.0 == *x.0).unwrap(), x.1))
            .collect();
        valve_map.push((s, f, v));
    }
    let start = valve_map.iter().position(|x| x.0 == "AA").unwrap();
let mut path = vec![start];
for i in valve_map {
if i.1 == 0 {
print!("{}: {}", i.0, i.2.len());
}
}

}
