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
        let mut a: Vec<(usize, usize)> = Vec::new();
        for (i, val) in letters.iter().enumerate() {
            for (j, val) in val.chars().enumerate() {
                if val == 'A' {
                    a.push((i, j));
                }
            }
        }
        let l = letters.len();
        for (p1, p2) in &a {
            if (*p1 > 0)
                && (*p2 > 0)
                && (*p1 < l - 1)
                && (*p2 < l - 1)
                && (letters[p1 - 1].chars().nth(p2 - 1).unwrap() == 'M')
                && (letters[p1 - 1].chars().nth(p2 + 1).unwrap() == 'S')
                && (letters[p1 + 1].chars().nth(p2 - 1).unwrap() == 'M')
                && (letters[p1 + 1].chars().nth(p2 + 1).unwrap() == 'S')
            {
                matches += 1;
            }
        }
        // Rotate 90 degrees.
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
