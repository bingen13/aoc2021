use std::cmp::Ordering;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Item {
    N(u32),
    O,
    C,
}

fn order(pair1: &[Item], pair2: &[Item]) -> bool {
    let mut p1 = Vec::new();
    let mut p2 = Vec::new();
    for i in 0..pair1.len() {
        p1.push(pair1[(pair1.len() - i) - 1]);
    }
    for i in 0..pair2.len() {
        p2.push(pair2[(pair2.len() - i) - 1]);
    }
    loop {
        let item1 = p1.pop().unwrap();
        let item2 = p2.pop().unwrap();
        if p1.is_empty() {
            return true;
        }
        if p2.is_empty() {
            return false;
        }
        match (item1, item2) {
            (Item::O, Item::O) => {}
            (Item::C, Item::C) => {}
            (Item::N(n1), Item::N(n2)) => {
                if n1 < n2 {
                    return true;
                } else if n1 > n2 {
                    return false;
                }
            }
            (Item::C, _) => {
                return true;
            }
            (_, Item::C) => {
                return false;
            }
            (Item::N(n), Item::O) => {
                p1.push(Item::C);
                p1.push(Item::N(n));
                p1.push(Item::O);
                p2.push(item2);
            }
            (Item::O, Item::N(n)) => {
                p2.push(Item::C);
                p2.push(Item::N(n));
                p2.push(Item::O);
                p1.push(item1);
            }
        }
    }
    false
}

fn parse(s: &str) -> Vec<Item> {
    let mut digit: Option<u32> = None;
    let mut v = Vec::new();
    for i in s.chars() {
        match i {
            '[' => v.push(Item::O),
            ']' => {
                if let Some(d) = digit {
                    v.push(Item::N(d));
                    digit = None;
                }
                v.push(Item::C);
            }
            ',' => {
                if let Some(d) = digit {
                    v.push(Item::N(d));
                    digit = None;
                }
            }
            '0'..='9' => {
                if let Some(mut d) = digit {
                    d *= 10;
                    d += i.to_digit(10).unwrap();
                    digit = Some(d);
                } else {
                    digit = Some(i.to_digit(10).unwrap());
                }
            }
            _ => println!("Well, fuck."),
        }
    }
    v
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n\n");
    let mut pairs = Vec::new();
    let divider1 = parse("[[2]]");
    let divider2 = parse("[[6]]");
    pairs.push(divider1.clone());
    pairs.push(divider2.clone());
    for i in f {
        let mut i = i.split('\n');
        let p1 = i.next().unwrap();
        let p2 = i.next().unwrap();
        let p1 = parse(p1);
        let p2 = parse(p2);
        pairs.push(p1);
        pairs.push(p2);
    }
    pairs.sort_by(|i1, i2| {
        if order(i1, i2) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    let d1 = pairs.iter().position(|x| x == &divider1).unwrap();
    let d2 = pairs.iter().position(|x| x == &divider2).unwrap();
    println!("{}", (d1 + 1) * (d2 + 1));
}
