use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
struct Bag {
    colour: String,
    contains: HashMap<String, u64>,
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
                cap[1].parse::<u64>().unwrap(),
            );
        }
        Bag {
            colour: colour,
            contains: map,
        }
    }

    fn inside(&self) -> u64 {
        if self.is_final() {
            return 0;
        } else {
            let mut c = 0;
            for (_, i) in &self.contains {
                c += i;
            }
            return c;
        }
    }

    fn is_final(&self) -> bool {
        self.contains.len() == 0
    }
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
    let s = "shiny gold".to_string();
    let b = bags.iter().filter(|c| c.colour == s).next().unwrap();
    let mut v1 = Vec::new();
    let mut v2 = HashMap::new();
    v1.push(b);
    while v1.len() > 0 {
        let cur = v1.pop().unwrap();

        for (i, j) in &cur.contains {
            v2.insert(i, j);
            v1.push(bags.iter().filter(|c| &c.colour == i).next().unwrap());
        }
    }

    println!("{}", count_boxes(&b, &bags));
}

fn count_boxes(b: &Bag, bags: &Vec<Bag>) -> u64 {
    let mut count: u64 = 0;
    let mut v = Vec::new();
    v.push((1, b));
    while v.len() > 0 {
        let (m, b) = v.pop().unwrap();
        if !b.is_final() {}
        count += b.inside() * m;
        for (i, j) in &b.contains {
            v.push((
                (j * m),
                &bags.iter().filter(|c| &c.colour == i).next().unwrap(),
            ));
        }
    }

    return count;
}
