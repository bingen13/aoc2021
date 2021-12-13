use std::collections::HashSet;
use std::fs::read_to_string;

// A point.
#[derive(Debug, Hash, Eq, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn fold(&self, instruction: &(bool, u32)) -> Point {
        let x;
        let y;
        let h = instruction.0;
        let l = instruction.1;
        if h {
            x = self.x;
            if self.y > l {
                y = (2 * l) - self.y;
            } else {
                y = self.y;
            }
        } else {
            y = self.y;
            if self.x > l {
                x = (2 * l) - self.x;
            } else {
                x = self.x;
            }
        }
        Point { x: x, y: y }
    }
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut f = f.split("\n\n");
    let points = f.next().unwrap();
    let points = points.split("\n");
    let instructions = f.next().unwrap();
    let mut sheet: HashSet<Point> = HashSet::new();
    for i in points {
        let mut p = i.split(",");
        sheet.insert(Point {
            x: p.next().unwrap().parse::<u32>().unwrap(),
            y: p.next().unwrap().parse::<u32>().unwrap(),
        });
    }
    let instructions = instructions.split("\n");
    let mut operations = Vec::new();
    for i in instructions {
        for j in i.split_whitespace().last() {
            if j.len() > 0 {
                let mut k = j.split("=");
                let horizontal = match k.next().unwrap() {
                    "x" => false,
                    "y" => true,
                    _ => {
                        println!("Error.");
                        false
                    }
                };
                let line = k.next().unwrap().parse::<u32>().unwrap();
                operations.push((horizontal, line));
            }
        }
    }
    for i in operations.iter() {
        sheet = sheet.iter().map(|p| p.fold(i)).collect();
    }
    printer(&sheet);
}

fn printer(s: &HashSet<Point>) {
    let m = s
        .iter()
        .map(|p| p.x)
        .reduce(|a, b| if a > b { a } else { b })
        .unwrap();
    let n = s
        .iter()
        .map(|p| p.y)
        .reduce(|a, b| if a > b { a } else { b })
        .unwrap();
    for i in 0..=n {
        for j in 0..=m {
            if s.contains(&Point { x: j, y: i }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}
