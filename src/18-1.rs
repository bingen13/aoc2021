use std::collections::HashSet;
use std::fs::read_to_string;

fn neighbours(cube: (u32, u32, u32), cubes: &HashSet<(u32, u32, u32)>) -> u32 {
    let mut n = 0;
    let x = cube.0;
    let y = cube.1;
    let z = cube.2;
    if x > 0 && cubes.contains(&(x - 1, y, z)) {
        n += 1;
    }
    if cubes.contains(&(x + 1, y, z)) {
        n += 1;
    }
    if y > 0 && cubes.contains(&(x, y - 1, z)) {
        n += 1;
    }
    if cubes.contains(&(x, y + 1, z)) {
        n += 1;
    }
    if z > 0 && cubes.contains(&(x, y, z - 1)) {
        n += 1;
    }
    if cubes.contains(&(x, y, z + 1)) {
        n += 1;
    }
    n
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let mut cubes = HashSet::new();
    for i in f {
        if i.is_empty() {
            break;
        }
        let i = i.split(',');
        if let [n1, n2, n3] = i.map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>()[..] {
            cubes.insert((n1, n2, n3));
        }
    }
    let mut edges = 0;
    for c in cubes.iter() {
        edges += 6 - neighbours(*c, &cubes);
    }
    println!("{}", edges);
}
