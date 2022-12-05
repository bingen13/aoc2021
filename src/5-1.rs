use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut f = f.split("\n\n");
    let stacks = f.next().unwrap();
    let moves = f.next().unwrap();
    let mut blocks = Vec::new();
    for _i in 0..9 {
        blocks.push(Vec::new());
    }

    for i in stacks.split('\n') {
        for (j, k) in i.chars().enumerate() {
            if (j % 4 == 1) & !k.is_numeric() & (k != ' ') {
                blocks[j / 4].push(k);
            }
        }
    }
    for i in 0..blocks.len() {
        blocks[i].reverse();
    }
    for i in moves.split('\n') {
        if !i.is_empty() {
            let mut i = i.split(' ');
            let n1 = i.nth(1).unwrap().parse::<usize>().unwrap();
            let n2 = i.nth(1).unwrap().parse::<usize>().unwrap();
            let n3 = i.nth(1).unwrap().parse::<usize>().unwrap();
            for _ in 0..n1 {
                let elem = blocks[n2 - 1].pop().unwrap();
                blocks[n3 - 1].push(elem);
            }
        }
    }
    let mut result = String::new();
    for i in 0..blocks.len() {
        result.push(blocks[i].pop().unwrap());
    }
    println!("{}", result);
}
