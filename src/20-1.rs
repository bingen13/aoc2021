use std::fs::read_to_string;

fn mix(v: &mut Vec<i64>, p: &mut Vec<usize>) {
    for i in 0..v.len() {
        let item = p.iter().position(|&x| x == i).unwrap();
        p.remove(item);
        p.insert((item as i64 + v[i]).rem_euclid(p.len() as i64) as usize, i);
    }
}

fn order(v: &[(usize, i64)]) -> Vec<i64> {
    const key: i64 = 811589153;
    let mut result = Vec::new();
    for i in 0..v.len() {
        result.push(v.iter().find(|p| p.0 == i).unwrap().1);
    }
    result
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let mut numbers = Vec::new();
    for i in f {
        if let Ok(i) = i.parse::<i64>() {
            numbers.push(i);
        }
    }
    let mut positions: Vec<_> = (0..numbers.len()).collect();
    mix(&mut numbers, &mut positions);
    let p = positions
        .iter()
        .position(|&x| x == numbers.iter().position(|&x| x == 0).unwrap())
        .unwrap();
    let pos1 = numbers[positions[(p + 1000) % positions.len()]];
    let pos2 = numbers[positions[(p + 2000) % positions.len()]];
    let pos3 = numbers[positions[(p + 3000) % positions.len()]];
    let result = pos1 + pos2 + pos3;
    println!("{}", result);
}
