use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let re_digits = Regex::new(r"(\d+)").unwrap();
    let re_op = Regex::new(r"new = old (\*|\+) (old|\d+)").unwrap();
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n\n");
    let mut v = Vec::new();
    for i in f {
        if i.is_empty() {
            break;
        }
        let mut i = i.split('\n');
        i.next();
        let items: Vec<_> = re_digits
            .captures_iter(i.next().unwrap())
            .map(|n| n[1].parse::<usize>().unwrap())
            .collect();
        let cap = re_op.captures(i.next().unwrap()).unwrap();
        let op = if &cap[2] == "old" {
            ("**".to_string(), 0)
        } else {
            (cap[1].to_string(), cap[2].parse::<usize>().unwrap())
        };
        let divisible = re_digits.captures(i.next().unwrap()).unwrap()[1]
            .parse::<usize>()
            .unwrap();
        let true_test = re_digits.captures(i.next().unwrap()).unwrap()[1]
            .parse::<usize>()
            .unwrap();
        let false_test = re_digits.captures(i.next().unwrap()).unwrap()[1]
            .parse::<usize>()
            .unwrap();
        v.push((items, op, divisible, true_test, false_test));
    }
    let mut inspect = Vec::new();
    for _ in 0..v.len() {
        inspect.push(0);
    }

    for _ in 0..20 {
        for i in 0..v.len() {
            let items = v[i].0.clone();
            let op = v[i].1.clone();
            let divisible = v[i].2;
            let true_test = v[i].3;
            let false_test = v[i].4;
            for j in 0..items.len() {
                inspect[i] += 1;
                let mut item = items[j];
                let (op, n) = op.clone();
                match op.as_str() {
                    "+" => item += n,
                    "*" => item *= n,
                    "**" => item *= item,
                    _ => (),
                }
                item /= 3;
                if item % divisible == 0 {
                    v[true_test].0.push(item);
                } else {
                    v[false_test].0.push(item);
                }
            }
            v[i].0 = Vec::new();
        }
    }
    inspect.sort();
    inspect.reverse();
    println!("{}", inspect[0] * inspect[1]);
}
