use std::fs::read_to_string;

fn main() {
    let mut letters = Vec::new();
    let f = read_to_string("input.txt").unwrap();
    let s = f.split('\n');
    for i in s {
        if i.is_empty() {
            break;
        }
        letters.push(i.to_string());
    }
    let key = "XMAS";
    let mut matches = 0;
    let mut x: Vec<(usize, usize)> = Vec::new();
    let l = letters.len();
    for (i, val) in letters.iter().enumerate() {
        for (j, val) in val.chars().enumerate() {
            if val == 'X' {
                x.push((i, j));
            }
        }
    }
    for _ in 0..4 {
        for (p1, p2) in &x {
            // Horizontals.
            if (*p2 <= l - 4) && (letters[*p1][*p2..(*p2 + 4)] == *key) {
                matches += 1;
            }
            // Diagonals.
            if (*p1 <= l - 4)
                && (*p2 <= l - 4)
                && ((0..4)
                    .map(|c| letters[p1 + c].chars().nth(p2 + c).unwrap())
                    .collect::<String>()
                    .as_str()
                    == key)
            {
                matches += 1;
            }
        }
        // Rotate 90 degrees anticlockwise.
        let mut v = Vec::new();
        for i in 0..l {
            let mut v2 = Vec::new();
            for j in &letters {
                v2.push(j.chars().nth(l - i - 1).unwrap());
            }
            v.push(v2.into_iter().collect::<String>());
        }
        letters = v;
        // Rotate positions 90 degrees anticlockwise.
        x.iter_mut().for_each(|(i, j)| {
            let i2 = l - *j - 1;
            let j2 = *i;
            (*i, *j) = (i2, j2);
        });
    }
    println!("{}", matches);
}
