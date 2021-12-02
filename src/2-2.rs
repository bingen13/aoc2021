use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let i: Vec<&str> = ip.split(" ").collect();
                match i[0] {
                    "forward" => {
                        x += i[1].parse::<u32>().unwrap();
                        y += aim * i[1].parse::<u32>().unwrap()
                    }
                    "down" => aim += i[1].parse::<u32>().unwrap(),
                    "up" => aim -= i[1].parse::<u32>().unwrap(),
                    _ => (),
                }
            }
        }
    } else {
        println!("File not found!")
    }

    println!("{}", x * y);
}
