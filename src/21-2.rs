use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::read_to_string;
use std::iter::once;

/// Transition from one character to another, finishing with pressing that character.
#[derive(Clone, Hash, Eq, PartialEq, Copy)]
struct Trans {
    origin: char,
    dest: char,
}

impl fmt::Display for Trans {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.origin, self.dest)
    }
}

impl fmt::Debug for Trans {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Trans {
    fn new(c1: char, c2: char) -> Self {
        Trans {
            origin: c1,
            dest: c2,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn pos<A: PartialEq + Copy, const N: usize, const M: usize>(
    m: &[[Option<A>; N]; M],
    t: &A,
) -> (usize, usize) {
    m.iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|x| x == &Some(*t)).map(|j| (i, j)))
        .unwrap()
}

fn md(p1: (usize, usize), p2: (usize, usize)) -> usize {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    (x1.max(x2) - x1.min(x2)) + (y1.max(y2) - y1.min(y2))
}

fn valid<A, const M: usize, const N: usize>(
    (s1, s2): (usize, usize),
    d: Dir,
    k: &[[Option<A>; N]; M],
) -> bool {
    match d {
        Dir::Up => s1 > 0 && k[s1 - 1][s2].is_some(),
        Dir::Down => s1 < M && k[s1 + 1][s2].is_some(),
        Dir::Left => s2 > 0 && k[s1][s2 - 1].is_some(),
        Dir::Right => s2 < N && k[s1][s2 + 1].is_some(),
    }
}

fn tcost<const N: usize, const M: usize>(
    t: Trans,
    k: &[[Option<char>; N]; M],
    hc: &HashMap<Trans, usize>,
) -> usize {
    let (x1, y1) = pos(k, &t.origin);
    let (x2, y2) = pos(k, &t.dest);
    let mut moves = Vec::new();
    if x1 > x2 {
        (0..x1 - x2).for_each(|_| moves.push(Dir::Up));
    } else if x1 < x2 {
        (0..x2 - x1).for_each(|_| moves.push(Dir::Down));
    }
    if y1 < y2 {
        (0..y2 - y1).for_each(|_| moves.push(Dir::Right));
    } else if y1 > y2 {
        (0..y1 - y2).for_each(|_| moves.push(Dir::Left));
    }
    let mut perms = Vec::new();
    permute(&mut moves, 0, &mut perms);
    perms
        .iter()
        .filter(|e| {
            e.iter()
                .try_fold((x1, y1), |(i, j), dir| {
                    if valid((i, j), *dir, k) {
                        match dir {
                            Dir::Up => Some((i - 1, j)),
                            Dir::Down => Some((i + 1, j)),
                            Dir::Left => Some((i, j - 1)),
                            Dir::Right => Some((i, j + 1)),
                        }
                    } else {
                        None
                    }
                })
                .is_some()
        })
        .map(|v| {
            v.iter()
                .map(|e| match e {
                    Dir::Up => '^',
                    Dir::Down => 'v',
                    Dir::Left => '<',
                    Dir::Right => '>',
                })
                .collect()
        })
        .map(|v: Vec<char>| {
            once(&'A')
                .chain(v.clone().iter())
                .zip(v.iter().chain(once(&'A')))
                .map(|(c1, c2)| Trans::new(*c1, *c2))
                .collect::<Vec<Trans>>()
        })
        .map(|v| v.iter().fold(0, |acc, e| acc + hc.get(e).unwrap()))
        .min()
        .unwrap()
}

fn permute(items: &mut Vec<Dir>, start: usize, results: &mut Vec<Vec<Dir>>) {
    if start == items.len() {
        results.push(items.clone());
        return;
    }
    let mut seen = HashSet::new();
    for i in start..items.len() {
        if seen.insert(items[i]) {
            items.swap(start, i);
            permute(items, start + 1, results);
            items.swap(start, i);
        }
    }
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut codes = Vec::new();
    for i in f.lines() {
        let mut v = Vec::new();
        for c in i.chars() {
            v.push(c);
        }
        codes.push(v);
    }
    let h: Vec<HashMap<Trans, usize>> = codes
        .iter()
        .map(|vec| {
            once(&'A')
                .chain(vec.iter())
                .zip(vec)
                .map(|(p1, p2)| Trans::new(*p1, *p2))
                .fold(HashMap::new(), |mut acc, pair| {
                    *acc.entry(pair).or_insert(0) += 1;
                    acc
                })
        })
        .collect();

    let numeric = [
        [Some('7'), Some('8'), Some('9')],
        [Some('4'), Some('5'), Some('6')],
        [Some('1'), Some('2'), Some('3')],
        [None, Some('0'), Some('A')],
    ];
    let keypad = [
        [None, Some('^'), Some('A')],
        [Some('<'), Some('v'), Some('>')],
    ];
    // Determine the cost of transitions on the first keyboard the robot uses, in terms of number of characters of the string I must type.
    // This is equal to the Manhattan distance +1 to press the final A.
    let cost1: HashMap<Trans, usize> = keypad
        .iter()
        .flat_map(|row1| row1.iter().flatten())
        .flat_map(|&e1| {
            keypad
                .iter()
                .flat_map(|row2| row2.iter().flatten())
                .map(move |&e2| Trans::new(e1, e2))
        })
        .map(|t| (t, md(pos(&keypad, &t.origin), pos(&keypad, &t.dest)) + 1))
        .collect();
    // Now calculate the cost for the 2nd directional keyboard the robot uses, based on previous.
    let mut cost2: HashMap<Trans, usize> = keypad
        .iter()
        .flat_map(|row1| row1.iter().flatten())
        .flat_map(|&e1| {
            keypad
                .iter()
                .flat_map(|row2| row2.iter().flatten())
                .map(move |&e2| Trans::new(e1, e2))
        })
        .map(|t| (t, tcost(t, &keypad, &cost1)))
        .collect();
    // Update the cost with the 23 intervening robot directional keyboards.
    for _ in 0..23 {
        cost2 = keypad
            .iter()
            .flat_map(|row1| row1.iter().flatten())
            .flat_map(|&e1| {
                keypad
                    .iter()
                    .flat_map(|row2| row2.iter().flatten())
                    .map(move |&e2| Trans::new(e1, e2))
            })
            .map(|t| (t, tcost(t, &keypad, &cost2)))
            .collect();
    }
    // Calculate the final cost for the numeric keypad.
    let cost3: HashMap<Trans, usize> = numeric
        .iter()
        .flat_map(|row1| row1.iter().flatten())
        .flat_map(|&e1| {
            numeric
                .iter()
                .flat_map(|row2| row2.iter().flatten())
                .map(move |&e2| Trans::new(e1, e2))
        })
        .map(|t| (t, tcost(t, &numeric, &cost2)))
        .collect();
    let hc: Vec<_> = h
        .iter()
        .map(|c| {
            c.iter()
                .fold(0, |acc, (t, n)| acc + cost3.get(t).unwrap() * n)
        })
        .collect();
    println!(
        "{}",
        codes
            .iter()
            .enumerate()
            .map(|(i, c)| {
                hc[i]
                    * c[..c.len() - 1]
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap()
            })
            .sum::<usize>()
    );
}
