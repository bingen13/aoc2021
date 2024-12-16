use std::collections::HashSet;
use std::fs::read_to_string;

fn print_map(
    l1: &usize,
    l2: &usize,
    walls: &HashSet<(usize, usize)>,
    boxes: &[Mybox],
    robot: &(usize, usize),
) {
    for i in 0..*l1 {
        for j in 0..*l2 {
            if walls.contains(&(i, j)) {
                print!("#");
            } else if boxes.iter().any(|b| (b.height == i) && (b.start == j)) {
                print!("[");
            } else if boxes.iter().any(|b| (b.height == i) && (b.end == j)) {
                print!("]");
            } else if *robot == (i, j) {
                print!("@");
            } else {
                print!(".");
            }
        }

        print!("\n");
    }
    print!("\n");
}

#[derive(Debug)]
struct Mybox {
    height: usize,
    start: usize,
    end: usize,
}

impl Mybox {
    fn new(height: usize, start: usize) -> Self {
        Mybox {
            height,
            start,
            end: start + 1,
        }
    }

    fn up(&mut self) {
        self.height -= 1;
    }

    fn down(&mut self) {
        self.height += 1;
    }

    fn left(&mut self) {
        self.start -= 1;
        self.end -= 1;
    }

    fn right(&mut self) {
        self.start += 1;
        self.end += 1;
    }

    fn overlaps(&self, p: &(usize, usize)) -> bool {
        (self.height == p.0) && ((self.start == p.1) || (self.end == p.1))
    }
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut f = f.split("\n\n");
    let old_tiles = f.next().unwrap();
    let mut tiles = String::new();
    for i in old_tiles.chars() {
        match i {
            '#' => tiles += "##",
            'O' => tiles += "[]",
            '\n' => tiles += "\n",
            '@' => tiles += "@.",
            '.' => tiles += "..",
            _ => todo!(),
        }
    }
    let mut s = tiles.split('\n');
    let l1 = s.clone().count();
    let l2 = s.next().unwrap().len();
    let mut walls = HashSet::new();
    let mut boxes = Vec::new();
    let mut robot = (0, 0);
    for (i, line) in tiles.split('\n').enumerate() {
        if line.is_empty() {
            break;
        }
        for (j, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    walls.insert((i, j));
                }
                '@' => {
                    robot = (i, j);
                }
                '[' => {
                    boxes.push(Mybox::new(i, j));
                }
                ']' | '.' => (),
                _ => todo!(),
            }
        }
    }
    let moves = f.next().unwrap();
    // print_map(&l1, &l2, &walls, &boxes, &robot);
    for m in moves.chars() {
        // println!("Move: {}", m);
        match m {
            '<' => {
                let wall = walls
                    .iter()
                    .filter(|w| (w.0 == robot.0) && (w.1 < robot.1))
                    .map(|w| w.1)
                    .max()
                    .unwrap();
                if let Some(hole) = (wall + 1..robot.1)
                    .filter(|h| {
                        !boxes
                            .iter()
                            .any(|b| (b.height == robot.0) && ((b.start == *h) || (b.end == *h)))
                    })
                    .max()
                {
                    for b in boxes
                        .iter_mut()
                        .filter(|b| (b.height == robot.0) && (b.start > hole) && (b.end < robot.1))
                    {
                        b.left();
                    }
                    robot.1 -= 1;
                }
            }

            '>' => {
                let wall = walls
                    .iter()
                    .filter(|w| (w.0 == robot.0) && (w.1 > robot.1))
                    .map(|w| w.1)
                    .min()
                    .unwrap();
                if let Some(hole) = (robot.1 + 1..wall)
                    .filter(|h| {
                        !boxes
                            .iter()
                            .any(|b| (b.height == robot.0) && ((b.start == *h) || (b.end == *h)))
                    })
                    .min()
                {
                    for b in boxes
                        .iter_mut()
                        .filter(|b| (b.height == robot.0) && (b.start > robot.1) && (b.end < hole))
                    {
                        b.right();
                    }
                    robot.1 += 1;
                }
            }

            '^' => {
                let wall = walls
                    .iter()
                    .filter(|w| (w.1 == robot.1) && (w.0 < robot.0))
                    .map(|w| w.0)
                    .max()
                    .unwrap();
                let mut movable: Vec<usize> = Vec::new();
                let mut points = Vec::new();
                points.push((robot.0 - 1, robot.1));
                //                for _ in wall + 1..robot.0 {
                loop {
                    if points.iter().any(|p| walls.contains(p)) {
                        break;
                    }
                    let obst = boxes
                        .iter()
                        .enumerate()
                        .filter(|(_, b)| points.iter().any(|p| b.overlaps(p)))
                        .map(|(index, _)| index)
                        .collect::<Vec<usize>>();
                    if obst.is_empty() {
                        movable.iter().for_each(|m| boxes[*m].up());
                        robot.0 -= 1;
                        break;
                    } else {
                        points = obst
                            .iter()
                            .flat_map(|m| {
                                vec![
                                    (boxes[*m].height - 1, boxes[*m].start),
                                    (boxes[*m].height - 1, boxes[*m].end),
                                ]
                            })
                            .collect();
                        movable.extend(obst);
                    }
                }
            }

            'v' => {
/*
                println!(
                    "Robot: {:?}, Boxes: {:?}, {:?}",
                    robot,
                    boxes,
                    boxes.iter().any(|b| b.overlaps(&robot))
                );
*/

                let wall = walls
                    .iter()
                    .filter(|w| (w.1 == robot.1) && (w.0 > robot.0))
                    .map(|w| w.0)
                    .min()
                    .unwrap();
                let mut movable: Vec<usize> = Vec::new();
                let mut points = Vec::new();
                points.push((robot.0 + 1, robot.1));
                //                for _ in robot.0 + 1..wall {
                loop {
                    if points.iter().any(|p| walls.contains(p)) {
                        break;
                    }
                    let obst = boxes
                        .iter()
                        .enumerate()
                        .filter(|(_, b)| points.iter().any(|p| b.overlaps(p)))
                        .map(|(index, _)| index)
                        .collect::<Vec<usize>>();
// println!("Points: {:?}, Obst: {:?}", points, obst);
                    if obst.is_empty() {
                        movable.iter().for_each(|m| boxes[*m].down());
                        robot.0 += 1;
                        break;
                    } else {
                        points = obst
                            .iter()
                            .flat_map(|m| {
                                vec![
                                    (boxes[*m].height + 1, boxes[*m].start),
                                    (boxes[*m].height + 1, boxes[*m].end),
                                ]
                            })
                            .collect();
                        movable.extend(obst);
                    }
                }
/*
                println!(
                    "Robot: {:?}, Boxes: {:?}, {:?}",
                    robot,
                    boxes,
                    boxes.iter().any(|b| b.overlaps(&robot))
                );
*/

            }

            _ => (),
        }
        // print_map(&l1, &l2, &walls, &boxes, &robot);
    }
    let mut result = 0;
    for b in &boxes {
        result += (100 * b.height) + b.start;
    }
    println!("{},{:?}", result, robot);
}
