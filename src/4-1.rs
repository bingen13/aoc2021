use std::fs::read_to_string;

fn main() {
    let mut letters = Vec::new();
    let f = read_to_string("input.txt").unwrap();
    let mut s = f.split('\n');
    while let Some(i) = s.next() {
        if i.is_empty() {
            break;
        }
        letters.push(i.to_string());
    }
    let key = "XMAS";
    let mut matches = 0;
    for r in 0..4 {
        let mut x: Vec<(usize, usize)> = Vec::new();
        for (i, val) in letters.iter().enumerate() {
            for (j, val) in val.chars().enumerate() {
                if val == 'X' {
                    x.push((i, j));
                }
            }
        }
        for (p1, p2) in &x {
            // Horizontals.
            if (*p2 <= letters.len() - 4) && (letters[*p1][*p2..(*p2 + 4)] == *key) {
                matches += 1;
            }
            // Diagonals.
            if (*p1 <= letters.len() - 4)
                && (*p2 <= letters.len() - 4)
                && ((0..4)
                    .map(|c| letters[p1 + c].chars().nth(p2 + c).unwrap())
                    .collect::<String>()
                    .as_str()
                    == key)
            {
                matches += 1;
            }
        }
        // Rotate 90 degrees.
        let l = letters.len();
        let mut v = Vec::new();
        for i in 0..l {
            let mut v2 = Vec::new();
            for j in 0..l {
                v2.push(letters[j].chars().nth(l - i - 1).unwrap());
            }
            v.push(v2.into_iter().collect::<String>());
        }
        letters = v;
    }
    println!("{}", matches);
}
