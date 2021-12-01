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
    // File hosts must exist in current path before this produces output
    let mut entries = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if let Ok(entry) = ip.parse::<i32>() {
                    entries.push(entry)
                }
            }
        }
    } else {
        println!("File not found!")
    }
    let mut prev = -1;
    let mut incs = 0;
    for i in entries {
        if prev != -1 && i > prev {
            incs += 1;
        }
        prev = i;
    }
    println!("{}", incs);
}
