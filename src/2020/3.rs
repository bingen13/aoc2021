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
                entries.push(ip)
            }
        }
    } else {
        println!("File not found!")
    }
    let mut trees = [[false as bool; 31]; 323];
    let mut x: usize = 0;
    for i in entries.iter() {
        let mut y: usize = 0;
        for j in i.chars() {
            if j == '#' {
                trees[x][y] = true
            }
            y += 1
        }
        x += 1
    }
    let mut count1: u64 = 0;
    let mut count2: u64 = 0;
    let mut count3: u64 = 0;
    let mut count4: u64 = 0;
    let mut count5: u64 = 0;
    x = 0;
    for i in &trees {
        if i[x % 31] {
            count1 += 1
        }
        x += 1;
    }
    x = 0;
    for i in &trees {
        if i[(x * 3) % 31] {
            count2 += 1
        }
        x += 1;
    }
    x = 0;
    for i in &trees {
        if i[(x * 5) % 31] {
            count3 += 1
        }
        x += 1;
    }
    x = 0;
    for i in &trees {
        if i[(x * 7) % 31] {
            count4 += 1
        }
        x += 1;
    }
    x = 0;
    for i in &trees {
        if (x % 2 == 0) && (i[(x / 2) % 31]) {
            count5 += 1
        }
        x += 1;
    }
    let total = count1 * count2 * count3 * count4 * count5;
    println!(
        "{}, {}, {}, {}, {}. Total: {}.",
        count1, count2, count3, count4, count5, total
    );
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
