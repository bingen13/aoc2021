use std::fs::read_to_string;

fn main() {
    let mut l1 = Vec::<u32>::new();
    let mut l2 = Vec::<u32>::new();
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    for i in f {
        if i.is_empty() {
            break;
        }
        let mut s = i.split_whitespace().map(|s| s.parse::<u32>().unwrap());
        l1.push(s.next().unwrap());
        l2.push(s.next().unwrap());
    }
    let mut similarity: u32 = 0;
    for i in l1.iter() {
        similarity += (l2.iter().filter(|n| *n == i).count() as u32) * i;
    }
    println!("{}", similarity);
}
