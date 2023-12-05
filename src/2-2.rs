use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
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
let (mut red, mut green, mut blue) = (0, 0, 0,);
        for j in s {
            let j = j.split(", ");
            for k in j {
                let mut k = k.split(" ");
                if let (Some(number), Some(colour)) = (k.next(), k.next()) {
                    let number = number.parse::<u32>().unwrap();
                    match colour {
                        "red" => {
                            if number > red {
                                red = number;
                            }
                        }
                        "green" => {
                            if number > green {
                                green = number;
                            }
                        }
                        "blue" => {
                            if number > blue {
                                blue = number;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        total += red*green*blue;
    }
    println!("{}", total);
}
