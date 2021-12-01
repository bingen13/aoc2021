use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Nav {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n");
    let r = Regex::new(r"(N|S|W|E|F|L|R)(\d*)").unwrap();
    let mut nav = Vec::new();

    for s in f {
        if s.len() > 1 {
            for cap in r.captures_iter(s) {
                match &cap[1] {
                    "N" => nav.push((Nav::North, cap[2].parse::<i32>().unwrap())),
                    "S" => nav.push((Nav::South, cap[2].parse::<i32>().unwrap())),
                    "E" => nav.push((Nav::East, cap[2].parse::<i32>().unwrap())),
                    "W" => nav.push((Nav::West, cap[2].parse::<i32>().unwrap())),
                    "L" => nav.push((Nav::Left, cap[2].parse::<i32>().unwrap())),
                    "R" => nav.push((Nav::Right, cap[2].parse::<i32>().unwrap())),
                    "F" => nav.push((Nav::Forward, cap[2].parse::<i32>().unwrap())),
                    _ => println!("error!"),
                }
            }
        }
    }

    let mut north = 0;
    let mut east = 0;
    let mut pn = 1;
    let mut pe = 10;

    for (i, j) in nav.iter() {
        match i {
            Nav::North => pn += j,
            Nav::South => pn -= j,
            Nav::East => pe += j,
            Nav::West => pe -= j,
            Nav::Left => {
                let (tn, te) = (pn, pe);
                match j {
                    90 => {
                        pe = -tn;
                        pn = te;
                    }
                    180 => {
                        pn = -tn;
                        pe = -te;
                    }
                    270 => {
                        pe = tn;
                        pn = -te;
                    }
                    _ => (),
                }
            }

            Nav::Right => {
                let (tn, te) = (pn, pe);
                match j {
                    90 => {
                        pe = tn;
                        pn = -te;
                    }
                    180 => {
                        pn = -tn;
                        pe = -te;
                    }
                    270 => {
                        pe = -tn;
                        pn = te;
                    }
                    _ => (),
                }
            }

            Nav::Forward => {
                north += pn * j;
                east += pe * j;
            }
        }
        println!(
            "East: {}. North: {}. Waypoint East: {}. Waypoint north: {}",
            east, north, pe, pn
        );
    }
    println!("{}, {}, {}", east, north, north.abs() + east.abs());
}
