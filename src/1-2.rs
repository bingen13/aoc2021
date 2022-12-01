use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n\n");
    let mut elves = Vec::new();
    for i in f {
        let mut elf = 0;
        let i = i.split("\n");
        for j in i {
            if j.len() > 1 {
                elf += j.parse::<u32>().unwrap();
            }
        }
        elves.push(elf);
    }
    elves.sort();
    elves.reverse();
    print!("Sum: {}.", elves[0] + elves[1] + elves[2]);
}
