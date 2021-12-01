use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    let mut entries = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
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
    entries.retain(|x| x < &2020);
    for i1 in entries.iter() {
        for i2 in entries.iter() {
            for i3 in entries.iter() {
                if (i1 != i2) & (i2 != i3) & (i1 + i2 + i3 == 2020) {
                    println!(
                        "Entry 1: {}. Entry 2: {}. Entry 3: {}. Sum: {}. Product: {}.",
                        i1,
                        i2,
                        i3,
                        i1 + i2 + i3,
                        i1 * i2 * i3
                    )
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
