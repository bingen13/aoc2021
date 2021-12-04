use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut f = f.split("\n\n");
    let mut numbers = Vec::new();
    for i in f.next().unwrap().split(",") {
        numbers.push(i.parse::<u32>().unwrap());
    }
    let mut boards = Vec::new();
    for i in f {
        let mut v = Vec::new();
        for j in i.split_whitespace() {
            v.push((j.parse::<u32>().unwrap(), false));
        }
        boards.push(v);
    }

    for i in &numbers {
        for b in boards.iter_mut() {
            for index in 0..b.len() {
                if b[index].0 == *i {
                    b[index] = (*i, true);
                    if bingo(b) {
                        let mut sum = 0;
                        for (j, _) in b.iter().filter(|&(_, val)| val == &false) {
                            sum += j;
                        }
                        println!("Sum: {}. Number: {}. Product: {}.", sum, i, sum * i);
                        return;
                    }
                }
            }
        }
    }
}

fn bingo(b: &[(u32, bool)]) -> bool {
    for i in 0..5 {
        if b.iter()
            .enumerate()
            .filter(|(n, (_, v))| (n >= &(i * 5)) && (n < &((i + 1) * 5)) && *v)
            .count()
            == 5
        {
            return true;
        }
        if b.iter()
            .enumerate()
            .filter(|(n, (_, v))| (n % 5 == i) && *v)
            .count()
            == 5
        {
            return true;
        }
    }
    return false;
}
