use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut v = Vec::new();
    for (i, ch) in f.chars().enumerate() {
        if !ch.is_digit(10) {
            break;
        }
        if i % 2 == 0 {
            v.push((Some(i / 2), ch.to_digit(10).unwrap()));
        } else {
            v.push((None, ch.to_digit(10).unwrap()));
        }
    }
    let mut v2 = v.iter().filter_map(|e| e.0).collect::<Vec<_>>();
    v2.sort();
    v2.reverse();
    for i in v2 {
        let right = v.iter().position(|e| e.0 == Some(i)).unwrap();
        if let Some(left) = v[..right]
            .iter()
            .position(|e| (e.0 == None) && (e.1 >= v[right].1))
        {
            if v[left].1 == v[right].1 {
                v.swap(left, right);
            } else {
                let n = v[left].1 - v[right].1;
                v[left].1 = n;
                v[right].0 = None;
                v.insert(left, (Some(i), v[right].1));
            }
        }
        while let Some((i, e)) = v.windows(2).enumerate().find(|(i, e)| e[0].0 == e[1].0) {
            v[i] = (v[i].0, v[i].1 + v[i + 1].1);
            v.remove(i + 1);
        }
    }
    let mut s = 0;
    let mut i = 0;
    for val in v {
        if val.0.is_some() {
            for j in 0..val.1 {
                s += (i + j) as usize * val.0.unwrap();
            }
        }
        i += val.1;
    }
    println!("{}", s);
}
