use std::collections::HashSet;
use std::fs::read_to_string;

fn contact(a: &(i32, i32), b: &(i32, i32)) -> bool {
    let (d1, d2) = dist(a, b);
    (-1..=1).contains(&d1) & (-1..=1).contains(&d2)
}

fn dist(a: &(i32, i32), b: &(i32, i32)) -> (i32, i32) {
    let (x1, y1) = a;
    let (x2, y2) = b;
    (x1 - x2, y1 - y2)
}

fn move_tail(head: &(i32, i32), tail: &mut (i32, i32)) {
    if !contact(head, tail) {
        let (dx, dy) = dist(head, tail);
        if dx == 0 {
            tail.1 = (tail.1 + head.1) / 2;
        } else if dy == 0 {
            tail.0 = (tail.0 + head.0) / 2;
        } else if dx.abs() > dy.abs() {
            tail.0 = (tail.0 + head.0) / 2;
            tail.1 = head.1;
        } else if dy.abs() > dx.abs() {
            tail.1 = (tail.1 + head.1) / 2;
            tail.0 = head.0;
        } else if (dx.abs() == 2) & (dy.abs() == 2) {
            tail.0 = (tail.0 + head.0) / 2;
            tail.1 = (tail.1 + head.1) / 2;
        } else {
            println!("Error!");
        }
    }
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut middle = Vec::new();
    for _ in 0..8 {
        middle.push((0, 0));
    }
    let mut tail_pos = HashSet::new();
    tail_pos.insert(tail);
    for i in f {
        let i: Vec<_> = i.split(' ').collect();
        if let [direction, distance] = i[..] {
            let distance = distance.parse::<i32>().unwrap();
            for _ in 0..distance {
                match direction {
                    "U" => head.1 += 1,
                    "D" => head.1 -= 1,
                    "L" => head.0 -= 1,
                    "R" => head.0 += 1,
                    _ => (),
                }
                for k in 0..middle.len() {
                    if k == 0 {
                        move_tail(&head, &mut middle[k]);
                    } else {
                        let (tx, ty) = middle[k - 1];
                        move_tail(&(tx, ty), &mut middle[k]);
                    }
                }
                move_tail(&middle[7], &mut tail);
                tail_pos.insert(tail);
            }
        }
    }
    println!("{}", tail_pos.len());
}
