use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    Seat,
    Floor,
    Person,
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n");
    let mut grid = Vec::new();
    let r = Regex::new(r"(L|\.|#)").unwrap();
    let mut rows = 0;
    for s in f {
        rows += 1;
        for i in r.captures_iter(s) {
            grid.push(match &i[1] {
                "L" => Cell::Seat,
                "#" => Cell::Person,
                _ => Cell::Floor,
            });
        }
    }
    rows -= 1;
    let columns = grid.len() / rows;
    println!(
        "A grid of {}x{} with {} members.",
        rows,
        columns,
        grid.len()
    );
    loop {
        let g2 = new_grid(&grid, columns);
        if g2 == grid {
            break;
        }
        grid = g2;
    }
    let mut count = 0;
    for i in grid {
        if i == Cell::Person {
            count += 1
        }
    }
    println!("{}", count);
}

fn show_grid(v: &Vec<Cell>, c: usize) {
    for i in 0..v.len() {
        if i % c != c - 1 {
            print!("{:?}({}) ", v[i], count_alive(&v, c, i));
        } else {
            println!("{:?}({})", v[i], count_alive(&v, c, i));
        }
    }
}

fn is_top(v: &Vec<Cell>, c: usize, i: usize) -> bool {
    i < c
}

fn is_left(v: &Vec<Cell>, c: usize, i: usize) -> bool {
    i % c == 0
}

fn is_bottom(v: &Vec<Cell>, c: usize, i: usize) -> bool {
    i >= v.len() - c
}

fn is_right(v: &Vec<Cell>, c: usize, i: usize) -> bool {
    i % c == c - 1
}

fn up(c: usize, i: usize) -> usize {
    i - c
}

fn down(c: usize, i: usize) -> usize {
    i + c
}

fn left(c: usize, i: usize) -> usize {
    i - 1
}

fn right(c: usize, i: usize) -> usize {
    i + 1
}

fn count_visible(v: &Vec<Cell>, c: usize, i: usize) -> u8 {
    let mut living = 0;
    // ul
    let mut i2 = i;
    while !is_top(v, c, i2) && !is_left(v, c, i2) {
        match v[up(c, left(c, i2))] {
            Cell::Person => {
                living += 1;
                break;
            }
            Cell::Seat => break,
            Cell::Floor => i2 = up(c, left(c, i2)),
        }
    }

    // uc
    let mut i2 = i;
    while !is_top(v, c, i2) {
        match v[up(c, i2)] {
            Cell::Person => {
                living += 1;
                break;
            }
            Cell::Seat => break,
            Cell::Floor => i2 = up(c, i2),
        }
    }

    // ur
    let mut i2 = i;
    while !is_top(v, c, i2) && !is_right(v, c, i2) {
        match v[up(c, right(c, i2))] {
            Cell::Person => {
                living += 1;
                break;
            }
            Cell::Seat => break,
            Cell::Floor => i2 = up(c, right(c, i2)),
        }
    }

    // cl
    let mut i2 = i;
    while !is_left(v, c, i2) {
        match v[left(c, i2)] {
            Cell::Person => {
                living += 1;
                break;
            }
            Cell::Seat => break,
            Cell::Floor => i2 = left(c, i2),
        }
    }

    // cr
    let mut i2 = i;
    while !is_right(v, c, i2) {
        match v[right(c, i2)] {
            Cell::Person => {
                living += 1;
                break;
            }
            Cell::Seat => break,
            Cell::Floor => i2 = right(c, i2),
        }
    }

    // bl
    let mut i2 = i;
    while !is_bottom(v, c, i2) && !is_left(v, c, i2) {
        match v[down(c, left(c, i2))] {
            Cell::Person => {
                living += 1;
                break;
            }
            Cell::Seat => break,
            Cell::Floor => i2 = down(c, left(c, i2)),
        }
    }

    // bc
    let mut i2 = i;
    while !is_bottom(v, c, i2) {
        match v[down(c, i2)] {
            Cell::Person => {
                living += 1;
                break;
            }
            Cell::Seat => break,
            Cell::Floor => i2 = down(c, i2),
        }
    }

    // br
    let mut i2 = i;
    while !is_bottom(v, c, i2) && !is_right(v, c, i2) {
        match v[down(c, right(c, i2))] {
            Cell::Person => {
                living += 1;
                break;
            }
            Cell::Seat => break,
            Cell::Floor => i2 = down(c, right(c, i2)),
        }
    }
    living
}

fn count_alive(v: &Vec<Cell>, c: usize, i: usize) -> u8 {
    let mut living = 0;

    // top-line.
    if i >= c {
        // Left.
        if i % c != 0 {
            match v[i - (c + 1)] {
                Cell::Person => living += 1,
                _ => (),
            }
        }
        // Centre.
        match v[i - c] {
            Cell::Person => living += 1,
            _ => (),
        }
        // Right.
        if i % c < c - 1 {
            match v[i - (c - 1)] {
                Cell::Person => living += 1,
                _ => (),
            }
        }
    }

    // Centre-line.
    // left.
    if i % c != 0 {
        match v[i - 1] {
            Cell::Person => living += 1,
            _ => (),
        }
    }
    // Right.
    if i % c < c - 1 {
        match v[i + 1] {
            Cell::Person => living += 1,
            _ => (),
        }
    }

    // Bottom-line.
    if i < v.len() - c {
        // Left.
        if i % c != 0 {
            match v[(i + c) - 1] {
                Cell::Person => living += 1,
                _ => (),
            }
        }
        // Centre.
        match v[i + c] {
            Cell::Person => living += 1,
            _ => (),
        }
        // Right.
        if i % c < c - 1 {
            match v[i + c + 1] {
                Cell::Person => living += 1,
                _ => (),
            }
        }
    }
    living
}

fn new_grid(v: &Vec<Cell>, c: usize) -> Vec<Cell> {
    let mut v2 = Vec::new();
    for i in 0..v.len() {
        match v[i] {
            Cell::Person if count_visible(&v, c, i) > 4 => v2.push(Cell::Seat),
            Cell::Seat if count_visible(&v, c, i) == 0 => v2.push(Cell::Person),
            _ => v2.push(v[i]).clone(),
        }
    }
    v2
}
