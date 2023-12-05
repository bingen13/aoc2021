use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let red = 12;
    let green = 13;
    let blue = 14;
    let mut total = 0;
    'lineloop: for i in f {
        if i.is_empty() {
            break;
        }
        let mut s = i.split(": ");
        let game = s
            .next()
            .unwrap()
            .split(" ")
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let s = s.next().unwrap().split("; ");
        for j in s {
            let j = j.split(", ");
            for k in j {
                let mut k = k.split(" ");
                if let (Some(number), Some(colour)) = (k.next(), k.next()) {
                    let number = number.parse::<u32>().unwrap();
                    match colour {
                        "red" => {
                            if number > red {
                                continue 'lineloop;
                            }
                        }
                        "green" => {
                            if number > green {
                                continue 'lineloop;
                            }
                        }
                        "blue" => {
                            if number > blue {
                                continue 'lineloop;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        total += game;
    }
    println!("{}", total);
}
