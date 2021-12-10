use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n");
    let mut scores = Vec::new();
    for i in f {
        let mut v = Vec::new();
        let mut corrupt = true;
        for c in i.chars() {
            corrupt = false;
            match c {
                '(' | '[' | '{' | '<' => {
                    v.push(reverse(c));
                }
                ')' | ']' | '}' | '>' => {
                    if v.len() > 0 && &c == v.last().unwrap() {
                        v.pop();
                    } else {
                        corrupt = true;
                        break;
                    }
                }
                _ => (),
            }
        }
        if corrupt == false {
            scores.push(v);
        }
    }
    let mut num: Vec<u64> = Vec::new();
    for s in scores.iter() {
        let mut n = 0;
        for i in s.iter().rev() {
            n *= 5;
            n += match i {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0,
            }
        }
        num.push(n);
    }
num.sort();
println!("{}", num[num.len()/2]);

}

fn reverse(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => c,
    }
}
