use std::fs::read_to_string;

#[derive(Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
    U1,
    U2,
    U3,
    Error,
}

fn score(p1: &Play, p2: &Play) -> u64 {
    let mut score = 0;
    match p1 {
        Play::Rock => match p2 {
            Play::U1 => {
                score += 3;
            }
            Play::U2 => {
                score += 4;
            }
            Play::U3 => {
                score += 8;
            }
            _ => (),
        },
        Play::Paper => match p2 {
            Play::U1 => {
                score += 1;
            }
            Play::U2 => {
                score += 5;
            }
            Play::U3 => {
                score += 9;
            }
            _ => (),
        },
        Play::Scissors => match p2 {
            Play::U1 => {
                score += 2;
            }
            Play::U2 => {
                score += 6;
            }
            Play::U3 => {
                score += 7;
            }
            _ => (),
        },
        _ => (),
    }
    return score;
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let mut strategy = Vec::new();
    for i in f {
        let j: Vec<_> = i.split(' ').collect();
        if j.len() > 1 {
            let p1 = match j[0].chars().next().unwrap() {
                'A' => Play::Rock,
                'B' => Play::Paper,
                'C' => Play::Scissors,
                _ => Play::Error,
            };
            let p2 = match j[1].chars().next().unwrap() {
                'X' => Play::U1,
                'Y' => Play::U2,
                'Z' => Play::U3,
                _ => Play::Error,
            };
            strategy.push((p1, p2));
        }
    }
    let mut result = 0;
    for (j, k) in strategy.iter() {
        result += score(j, k);
    }
    println!("{:?}", result);
}
