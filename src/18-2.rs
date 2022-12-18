use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn neighbours_xy(cube: (u32, u32, u32), cubes: &HashSet<(u32, u32, u32)>) -> u32 {
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
    4 - n
}

fn neighbours_xz(cube: (u32, u32, u32), cubes: &HashSet<(u32, u32, u32)>) -> u32 {
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
    if z > 0 && cubes.contains(&(x, y, z - 1)) {
        n += 1;
    }
    if cubes.contains(&(x, y, z + 1)) {
        n += 1;
    }
    4 - n
}

fn neighbours_yz(cube: (u32, u32, u32), cubes: &HashSet<(u32, u32, u32)>) -> u32 {
    let mut n = 0;
    let x = cube.0;
    let y = cube.1;
    let z = cube.2;
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
    4 - n
}

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
    let mut xy: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
    let mut xz: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
    let mut yz: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
    for c in cubes.iter() {
        let (x, y, z) = c;
        if let Some(oxy) = xy.get_mut(&(*x, *y)) {
            if z < &oxy.0 {
                (*oxy).0 = *z;
            }
            if z > &oxy.1 {
                (*oxy).1 = *z;
            }
        } else {
            xy.insert((*x, *y), (*z, *z));
        }
        if let Some(oxz) = xz.get_mut(&(*x, *z)) {
            if y < &oxz.0 {
                (*oxz).0 = *y;
            }
            if y > &oxz.1 {
                (*oxz).1 = *y;
            }
        } else {
            xz.insert((*x, *z), (*y, *y));
        }
        if let Some(oyz) = yz.get_mut(&(*y, *z)) {
            if x < &oyz.0 {
                (*oyz).0 = *x;
            }
            if x > &oyz.1 {
                (*oyz).1 = *x;
            }
        } else {
            yz.insert((*y, *z), (*x, *x));
        }
    }
    println!("{}, {}, {}.", xy.len(), xz.len(), yz.len());
    let mut checked = HashMap::new();
    for (i, j) in xy.iter() {
        let (x, y) = i;
        let (z1, z2) = j;
        checked.insert((x, y, z1), vec![neighbours_xy((*x, *y, *z1), &cubes)]);
        checked.insert((x, y, z2), vec![neighbours_xy((*x, *y, *z2), &cubes)]);
    }
    for (i, j) in xz.iter() {
        let (x, z) = i;
        let (y1, y2) = j;
        if let Some(v) = checked.get_mut(&(x, y1, z)) {
            v.push(neighbours_xz((*x, *y1, *z), &cubes));
        } else {
            checked.insert((x, y1, z), vec![neighbours_xz((*x, *y1, *z), &cubes)]);
        }
        if let Some(v) = checked.get_mut(&(x, y2, z)) {
            v.push(neighbours_xz((*x, *y2, *z), &cubes));
        } else {
            checked.insert((x, y2, z), vec![neighbours_xz((*x, *y2, *z), &cubes)]);
        }
    }
    for (i, j) in yz.iter() {
        let (y, z) = i;
        let (x1, x2) = j;
        if let Some(v) = checked.get_mut(&(x1, y, z)) {
            v.push(neighbours_xy((*x1, *y, *z), &cubes));
        } else {
            checked.insert((x1, y, z), vec![neighbours_xy((*x1, *y, *z), &cubes)]);
        }
        if let Some(v) = checked.get_mut(&(x2, y, z)) {
            v.push(neighbours_xy((*x2, *y, *z), &cubes));
        } else {
            checked.insert((x2, y, z), vec![neighbours_xy((*x2, *y, *z), &cubes)]);
        }
    }
    println!("{}", checked.len());
    edges = 0;
    for (_, i) in checked {
        edges += i.iter().max().unwrap()+1;
    }
    println!("{}", edges);
}
