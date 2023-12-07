use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut f = f.split("\n\n");
    let re = Regex::new(r"\d+").unwrap();
    let seeds = re
        .find_iter(f.next().unwrap())
        .map(|x| x.as_str().parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut ranges = Vec::new();

    for i in f {
        if i.is_empty() {
            break;
        }
        let mut r = HashMap::new();
        for j in i.split('\n') {
            if let [dest, source, lenght] = re
                .find_iter(j)
                .map(|x| x.as_str().parse::<u64>().unwrap())
                .collect::<Vec<_>>()[..]
            {
                r.insert(source..(source + lenght), dest);
            }
        }
        ranges.push(r.clone());
    }
    let mut locations = Vec::new();

    for i in seeds {
        let mut location = i;
        for j in &ranges {
            if let Some((key, value)) = j.iter().find(|(k, _)| k.contains(&location)) {
                location = value + (location - key.start);
            }
        }
        locations.push(location);
    }
    println!("{}", locations.iter().min().unwrap());
}
