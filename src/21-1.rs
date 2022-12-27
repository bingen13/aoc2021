use std::fs::read_to_string;

enum Op {
    Plus,
    Minus,
    Times,
    Div,
}

enum Monkey {
    Num(i64),
    Op(Op, String, String),
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let mut monkeys = Vec::new();
    for i in f {
        if i.is_empty() {
            break;
        }
        if let [name, i @ ..] = &i.split(' ').collect::<Vec<_>>()[..] {
            let monkey = match i {
                [n] => Monkey::Num(n.parse().unwrap()),
                [m1, "+", m2] => Monkey::Op(Op::Plus, m1.to_string(), m2.to_string()),
                [m1, "-", m2] => Monkey::Op(Op::Minus, m1.to_string(), m2.to_string()),
                [m1, "*", m2] => Monkey::Op(Op::Times, m1.to_string(), m2.to_string()),
                [m1, "/", m2] => Monkey::Op(Op::Div, m1.to_string(), m2.to_string()),
                _ => {
                    println!("Error.");
                    Monkey::Num(0)
                }
            };
            monkeys.push((name.split(':').next().unwrap(), monkey));
        }
    }
    loop {
        if let Monkey::Num(_) = monkeys.iter().find(|x| x.0 == "root").unwrap().1 {
            break;
        }
        for i in 0..monkeys.len() {
            if let Monkey::Op(o, m1, m2) = &monkeys[i].1 {
                let m1 = &monkeys.iter().find(|x| x.0 == m1).unwrap().1;
                let m2 = &monkeys.iter().find(|x| x.0 == m2).unwrap().1;
                if let Monkey::Num(n1) = m1 {
                    if let Monkey::Num(n2) = m2 {
                        match &o {
                            Op::Plus => {
                                monkeys[i].1 = Monkey::Num(n1 + n2);
                            }
                            Op::Minus => {
                                monkeys[i].1 = Monkey::Num(n1 - n2);
                            }
                            Op::Times => {
                                monkeys[i].1 = Monkey::Num(n1 * n2);
                            }
                            Op::Div => {
                                monkeys[i].1 = Monkey::Num(n1 / n2);
                            }
                        }
                    }
                }
            }
        }
    }
    if let Monkey::Num(n) = monkeys.iter().find(|x| x.0 == "root").unwrap().1 {
        println!("{}", n);
    }
}
