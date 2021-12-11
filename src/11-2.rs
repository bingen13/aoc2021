use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n");
    let mut energy = Vec::new();
    for i in f {
        if i.len() == 0 {
            break;
        }
        for j in i.chars() {
            energy.push(j.to_digit(10).unwrap());
        }
    }
    let mut steps = 0;
    loop {
        steps += 1;
        let f;
        f = step(energy);
        energy = f.0;
        if f.1 == 100 {
            break;
        }
    }

    println!("steps: {}.", steps);
}

fn step(mut v: Vec<u32>) -> (Vec<u32>, u32) {
    let mut flashes = 0;
    for i in v.iter_mut() {
        *i += 1;
    }
    while let Some(i) = v.iter().position(|elem| elem > &9) {
        // println!("I: {}", &i);
        let top = i < 10;
        let left = i % 10 == 0;
        let right = i % 10 == 9;
        let bottom = i > 89;
        flashes += 1;
        v[i] = 0;
        // Switch one row up.
        if !top {
            if !left {
                if v[i - 11] > 0 {
                    v[i - 11] += 1;
                }
            }
            if v[i - 10] > 0 {
                v[i - 10] += 1;
            }
            if !right {
                if v[i - 9] > 0 {
                    v[i - 9] += 1;
                }
            }
        }
        // Same row.
        if !left {
            if v[i - 1] > 0 {
                v[i - 1] += 1;
            }
        }
        if !right {
            if v[i + 1] > 0 {
                v[i + 1] += 1;
            }
        }
        // One row down.
        if !bottom {
            if !left {
                if v[i + 9] > 0 {
                    v[i + 9] += 1;
                }
            }
            if v[i + 10] > 0 {
                v[i + 10] += 1;
            }
            if !right {
                if v[i + 11] > 0 {
                    v[i + 11] += 1;
                }
            }
        }
    }
    /*
        for i in 0..100 {
            print!("{}", &v[i]);
            if i % 10 == 9 {
                print!("\n");
            }
        }
    */
    return (v, flashes);
}
