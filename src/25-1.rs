use std::fs::read_to_string;

fn transform(n: i64) -> String {
    let mut s = String::new();
    let mut num = n;
    if num == 0 {
        return "0".to_string();
    }
    let mut carry = 0;
    while num != 0 {
        let d = (num % 5) + carry;
        carry = 0;
        match d {
            0 => {
                s.push('0');
            }
            1 => {
                s.push('1');
            }
            2 => {
                s.push('2');
            }
            3 => {
                s.push('=');
                carry = 1;
            }
            4 => {
                s.push('-');
                carry = 1;
            }
            5 => {
                s.push('0');
                carry = 1;
            }
            _ => {
                println!("Error!");
            }
        }
        num = num / 5;
    }
    return s;
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let mut numbers = Vec::new();
    for i in f {
        if i.is_empty() {
            break;
        }
        let mut result: i64 = 0;
        for j in i.chars() {
            result *= 5;
            match j {
                '1' => {
                    result += 1;
                }
                '2' => {
                    result += 2;
                }
                '0' => {}
                '-' => {
                    result -= 1;
                }
                '=' => {
                    result -= 2;
                }
                _ => {
                    println!("Error!");
                }
            }
        }
        numbers.push(result);
    }
    let result = numbers.iter().sum::<i64>();
    println!(
        "{}, {}.",
        result,
        transform(result).chars().rev().collect::<String>()
    );
}
