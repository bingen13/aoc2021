use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let mut n = 0;
    for i in f {
        if i.is_empty() {
            break;
        }
        let mut nums = Vec::new();
        for j in i.chars().filter(|c| c.is_digit(10)) {
            nums.push(j.to_digit(10).unwrap());
        }
        n += (10 * nums.first().unwrap()) + nums.last().unwrap();
    }
    println!("{}", n);
}
