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

/*
fn assign(p1: &Play, i: usize) -> Play {
    if i == 0 {
        return match p1 {
            Play::U1 => Play::Rock,
            Play::U2 => Play::Paper,
            Play::U3 => Play::Scissors,
            _ => Play::Error,
        };
    }
    if i == 1 {
        return match p1 {
            Play::U1 => Play::Rock,
            Play::U2 => Play::Scissors,
            Play::U3 => Play::Paper,
            _ => Play::Error,
        };
    }
    if i == 2 {
        return match p1 {
            Play::U1 => Play::Paper,
            Play::U2 => Play::Rock,
            Play::U3 => Play::Scissors,
            _ => Play::Error,
        };
    }
    if i == 3 {
        return match p1 {
            Play::U1 => Play::Paper,
            Play::U2 => Play::Scissors,
            Play::U3 => Play::Rock,
            _ => Play::Error,
        };
    }
    if i == 4 {
        return match p1 {
            Play::U1 => Play::Scissors,
            Play::U2 => Play::Rock,
            Play::U3 => Play::Paper,
            _ => Play::Error,
        };
    }
    if i == 5 {
        return match p1 {
            Play::U1 => Play::Scissors,
            Play::U2 => Play::Paper,
            Play::U3 => Play::Rock,
            _ => Play::Error,
        };
    }
    return Play::Error;
}
*/

fn score(p1: &Play, p2: &Play) -> u64 {
    let mut score = 0;
    match p2 {
        Play::Rock => {
            score += 1;
            match p1 {
                Play::Rock => {
                    score += 3;
                }
                Play::Scissors => {
                    score += 6;
                }
                _ => (),
            }
        }
        Play::Paper => {
            score += 2;
            match p1 {
                Play::Paper => {
                    score += 3;
                }
                Play::Rock => {
                    score += 6;
                }
                _ => (),
            }
        }
        Play::Scissors => {
            score += 3;
            match p1 {
                Play::Scissors => {
                    score += 3;
                }
                Play::Paper => {
                    score += 6;
                }
                _ => (),
            }
        }
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
                'X' => Play::Rock,
                'Y' => Play::Paper,
                'Z' => Play::Scissors,
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
