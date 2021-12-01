use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

#[derive(Debug)]
struct Bag {
    colour: String,
    contains: HashMap<String, u32>,
}

impl Bag {
    // New bag.
    fn new(s: String) -> Bag {
        let colour_reg = Regex::new(r"^([[:alpha:]]*) ([[:alpha:]]*) bags contain").unwrap();
        let bags_reg =
            Regex::new(r"(?:(?:contain )|(?:, ))(\d) ([[:alpha:]]*) ([[:alpha:]]*) bag").unwrap();
        let mut colour = "FAIL FAIL".to_string();
        let mut map = HashMap::new();
        for cap in colour_reg.captures_iter(&s) {
            colour = format!("{} {}", &cap[1], &cap[2]);
        }
        for cap in bags_reg.captures_iter(&s) {
            map.insert(
                format!("{} {}", &cap[2], &cap[3]),
                cap[1].parse::<u32>().unwrap(),
            );
        }
        Bag {
            colour: colour,
            contains: map,
        }
    }

    fn contains(&self, s: &str) -> bool {
        self.contains.contains_key(s)
    }

    /*
        fn contains(&self, h: HashMap<&str, &u32>) -> bool {
            for (s, min) in h {
                if !self.contains.contains_key(s) || self.contains[s] < *min {
                    return false;
                }
            }
            true
        }
    */
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n");
    let mut bags = Vec::new();
    for i in f {
        if i != "" {
            bags.push(Bag::new(i.to_string()));
        }
    }
    let mut count = 0;
    let mut req = Vec::new();
    req.push("shiny gold".to_string());
    let mut outermost = HashSet::new();
    while req.len() > 0 {
        let element = req.pop().unwrap();
        let holders = bags.iter().filter(|c| c.contains(&element));
        for k in holders {
            println!("Element: {}. Holder: {}", &element, &k.colour);
            req.push(k.colour.clone());
        outermost.insert(k.colour.clone());
        }
    }

    println!("{}", outermost.iter().count());
}
