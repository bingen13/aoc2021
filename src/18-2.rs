use std::collections::{HashMap, HashSet};
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
            cubes.insert((n1 + 1, n2 + 1, n3 + 1));
        }
    }
    let mut edges = 0;
    for c in cubes.iter() {
        edges += 6 - neighbours(*c, &cubes);
    }
    println!("{}", edges);
    let mut xy: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
    let mut xz: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
    let mut yz: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
    for c in cubes.iter() {
        let (x, y, z) = c;
        if let Some(oxy) = xy.get_mut(&(*x, *y)) {
            if z < &oxy.0 {
                oxy.0 = *z;
            }
            if z > &oxy.1 {
                oxy.1 = *z;
            }
        } else {
            xy.insert((*x, *y), (*z, *z));
        }
        if let Some(oxz) = xz.get_mut(&(*x, *z)) {
            if y < &oxz.0 {
                oxz.0 = *y;
            }
            if y > &oxz.1 {
                oxz.1 = *y;
            }
        } else {
            xz.insert((*x, *z), (*y, *y));
        }
        if let Some(oyz) = yz.get_mut(&(*y, *z)) {
            if x < &oyz.0 {
                oyz.0 = *x;
            }
            if x > &oyz.1 {
                oyz.1 = *x;
            }
        } else {
            yz.insert((*y, *z), (*x, *x));
        }
    }
    edges = 0;
    for c in cubes {
        let (x, y, z) = c;
        let mut points = Vec::new();
        if x > 0 {
            points.push((x - 1, y, z));
        }
        points.push((x + 1, y, z));
        if y > 0 {
            points.push((x, y - 1, z));
        }
        points.push((x, y + 1, z));
        if z > 0 {
            points.push((x, y, z - 1));
        }
        points.push((x, y, z + 1));
        for p in points {
            let (x, y, z) = p;
            if let Some(zz) = xy.get(&(x, y)) {
                if (z < zz.0) || (z > zz.1) {
                    edges += 1;
                    continue;
                }
            } else {
                edges += 1;
                continue;
            }
            if let Some(yy) = xz.get(&(x, z)) {
                if (y < yy.0) || (y > yy.1) {
                    edges += 1;
                    continue;
                }
            } else {
                edges += 1;
                continue;
            }
            if let Some(xx) = yz.get(&(y, z)) {
                if (x < xx.0) || (x > xx.1) {
                    edges += 1;
                    continue;
                }
            } else {
                edges += 1;
                continue;
            }
        }
    }
    println!("{}", edges);
}
