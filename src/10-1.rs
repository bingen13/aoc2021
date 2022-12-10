use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let mut cycle = 1;
    let mut value: i32 = 1;
    let mut state = Vec::new();
    state.push((1, 1));
    for i in f {
        let i: Vec<_> = i.split(' ').collect();
        match i[..] {
            ["noop"] => {
                cycle += 1;
                state.push((cycle, value));
            }
            ["addx", val] => {
                cycle += 1;
                state.push((cycle, value));
                cycle += 1;
                value += val.parse::<i32>().unwrap();
                state.push((cycle, value));
            }
            _ => (),
        }
    }
    let mut result = 0;
    for (i, j) in state {
        if (i == 20) | ((i > 20) & ((i - 20) % 40 == 0)) {
            result += i * j;
        }
    }

    println!("{}", result);
}
