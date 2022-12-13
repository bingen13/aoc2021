use std::fs::read_to_string;

#[derive(Debug)]
enum Item {
    N(u32),
    O,
    C,
}

enum Elem {
    List(Vec<Elem>),
    Digit(u32),
    Error,
}

fn norm(p: &[Item]) -> Elem {
    let r: Elem;
    match p {
        [] => {
            r = Elem::Error;
        }
        [n] => {
            if let Item::N(m) = n {
                r = Elem::Digit(*m);
            } else {
                r = Elem::Error;
            }
        }
        [Item::C, _] => {
            r = Elem::Error;
        }
        [Item::O, Item::C] => {
            r = Elem::List(vec![]);
        }
        [Item::O, rest @ .., Item::C] => {
            r = Elem::List(vec![norm(rest)]);
        }
[Item::N(n), rest @ ..] => {
let mut v = vec![n];
let v2 = vec![norm(rest)];
v.extend(v2);
r = 


    }
    Elem::Error
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
    for i in f {
        let mut i = i.split('\n');
        let p1 = i.next().unwrap();
        let p2 = i.next().unwrap();
        let p1 = parse(p1);
        let p2 = parse(p2);
        println!("{:?}", p2);
    }
}
