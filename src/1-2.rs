use std::fs::read_to_string;

fn nums_rev(n: &[&str]) -> Vec<String> {
    let mut v = Vec::new();
    for i in n {
        v.push(i.chars().rev().collect::<String>());
    }
    v
}

fn get_first(s: &String, n: &[&str]) -> u32 {
    for i in 0..s.len() {
        let s = s.get(i..).unwrap();
        if s.starts_with(|c: char| c.is_digit(10)) {
            return s.chars().next().unwrap().to_digit(10).unwrap() as u32;
        }
        for (j, k) in n.iter().enumerate() {
            if s.starts_with(k) {
                return (j + 1) as u32;
            }
        }
    }
    0
}

fn main() {
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let mut n = 0;
    for i in f {
        if i.is_empty() {
            break;
        }
        let mut nums = Vec::new();
        let r = i.chars().rev().collect::<String>();
        let first = get_first(&i.to_string(), &numbers);
        let last = get_first(
            &r,
            &nums_rev(&numbers)
                .iter()
                .map(|x| x.as_str())
                .collect::<Vec<&str>>(),
        );
        nums.push(first);
        nums.push(last);
        n += (10 * nums.first().unwrap()) + nums.last().unwrap();
    }
    println!("{}", n);
}
