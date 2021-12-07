use std::fs::read_to_string;

fn main() {
    let mut f = read_to_string("input.txt").unwrap();
    f.retain(|c| !c.is_whitespace());
    let f = f.split(",");
    let mut positions = Vec::new();
    for i in f {
        positions.push(i.parse::<i64>().unwrap());
    }
    positions.sort();

    let mut fv = Vec::new();
    for i in positions[0]..=positions[positions.len() - 1] {
        let mut fuel = 0;

        for j in positions.iter() {
            fuel += calc(i, *j);
        }
        fv.push(fuel);
    }
    println!("{:?}", fv.iter().min().unwrap());
}

fn calc(a: i64, b: i64) -> i64 {
    let x = (a - b).abs();
    (x * (x + 1)) / 2
}
