use std::fs::read_to_string;

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut f = f.split("\n\n");
    let tiles = f.next().unwrap();
    let mut map = Vec::new();
    let mut line = Vec::new();
    for i in tiles.chars() {
        if i == '\n' {
            map.push(line);
            line = vec![];
        } else {
            line.push(i);
        }
    }
    map.push(line);
    for i in f.next().unwrap().chars() {
        let rh = map
            .iter()
            .filter_map(|e| e.iter().position(|e2| *e2 == '@'))
            .next()
            .unwrap();
        let rv = map.iter().position(|e| e.contains(&'@')).unwrap();
        match i {
            'v' => {
                let wall = map[rv + 1..]
                    .iter()
                    .position(|e| e[rh] == '#')
                    .map(|e| e + rv + 1)
                    .unwrap();
                if let Some(hole) = map[rv + 1..wall]
                    .iter()
                    .position(|e| e[rh] == '.')
                    .map(|e| e + rv + 1)
                {
                    let t = map[rv + 1][rh];
                    map[hole][rh] = t;
                    map[rv][rh] = '.';
                    map[rv + 1][rh] = '@';
                }
            }

            '>' => {
                let wall = map[rv][rh + 1..]
                    .iter()
                    .position(|e| *e == '#')
                    .map(|e| e + rh + 1)
                    .unwrap();
                if let Some(hole) = map[rv][rh + 1..wall]
                    .iter()
                    .position(|e| *e == '.')
                    .map(|e| e + rh + 1)
                {
                    let t = map[rv][rh + 1];
                    map[rv][hole] = t;
                    map[rv][rh] = '.';
                    map[rv][rh + 1] = '@';
                }
            }

            '<' => {
                let wall = map[rv][..rh]
                    .iter()
                    .enumerate()
                    .filter(|e| *e.1 == '#')
                    .max_by_key(|k| k.0)
                    .map(|e| e.0)
                    .unwrap();
                if let Some(hole) = map[rv][wall..=rh - 1]
                    .iter()
                    .enumerate()
                    .filter(|e| *e.1 == '.')
                    .max_by_key(|k| k.0)
                    .map(|k| k.0 + wall)
                {
                    let t = map[rv][rh - 1];
                    map[rv][hole] = t;
                    map[rv][rh] = '.';
                    map[rv][rh - 1] = '@';
                }
            }

            '^' => {
                let wall = map[..=rv - 1]
                    .iter()
                    .enumerate()
                    .filter(|e| e.1[rh] == '#')
                    .max_by_key(|k| k.0)
                    .map(|e| e.0)
                    .unwrap();
                if let Some(hole) = map[wall..=rv - 1]
                    .iter()
                    .enumerate()
                    .filter(|e| e.1[rh] == '.')
                    .max_by_key(|k| k.0)
                    .map(|e| e.0 + wall)
                {
                    let t = map[rv - 1][rh];
                    map[hole][rh] = t;
                    map[rv][rh] = '.';
                    map[rv - 1][rh] = '@';
                }
            }

            _ => (),
        }
        /*
                println!("Map:");
                for i in &map {
                    for j in i {
                        print!("{}", j);
                    }
                    print!("\n");
                }
                print!("\n");
        */
    }
    let mut result = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'O' {
                result += (100 * i) + j;
            }
        }
    }
    println!("{}", result);
}
