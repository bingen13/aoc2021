use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n");
    let mut values = Vec::new();
    for i in f {
        if i.len() == 0 {
            break;
        }
        let mut v = Vec::new();
        for j in i.chars() {
            v.push(j.to_digit(10).unwrap());
        }
        values.push(v);
    }
    let mut sum = 0;
    for i in 0..values.len() {
        for j in 0..values[0].len() {
            if is_min(&values, i, j) {
                sum += values[i][j] + 1;
            }
        }
    }
    println!("{}", sum);
}

fn is_min(v: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let l = v.len();
    if x > 0 && v[x][y] >= v[x - 1][y] {
        return false;
    }
    if !is_min_x(&v[x], y) {
        return false;
    }
    if x < (l - 1) && v[x][y] >= v[x + 1][y] {
        return false;
    }
    return true;
}

fn is_min_x(v: &Vec<u32>, x: usize) -> bool {
    let l = v.len();
    if x > 0 && v[x] >= v[x - 1] {
        return false;
    }
    if x < (l - 1) && v[x] >= v[x + 1] {
        return false;
    }
    return true;
}
