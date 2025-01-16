use std::collections::HashSet;
use std::fs::read_to_string;

fn conns<'a>(s: &str, h: &'a HashSet<(&'a str, &'a str)>) -> HashSet<&'a str> {
    h.iter()
        .filter(|(s1, s2)| s == *s1 || s == *s2)
        .map(|(s1, s2)| if s == *s1 { *s2 } else { *s1 })
        .collect()
}

fn main() {
    let f = read_to_string("input.txt").unwrap();
    let mut graph = HashSet::new();
    for line in f.lines() {
        let mut line = line.split('-');
        let (first, second) = (line.next().unwrap(), line.next().unwrap());
        if first < second {
            graph.insert((first, second));
        } else {
            graph.insert((second, first));
        }
    }
    let computers: HashSet<_> = graph.iter().flat_map(|(s1, s2)| [s1, s2]).collect();
    let mut set = HashSet::new();
    for c in computers
        .iter()
        .filter(|&&c| c.starts_with('t') && conns(c, &graph).len() > 1)
    {
        let mut target: Vec<&str> = vec![*c];
        let peers = conns(c, &graph);
        while let Some(c2) = peers
            .iter()
            .find(|&&x| {
                let peers2 = conns(x, &graph);
                target.iter().all(|&y| peers2.contains(y))
            })
            .cloned()
        {
            target.push(c2);
        }
        target.sort();
        set.insert(target);
    }
    println!("{}", set.iter().max_by_key(|x| x.len()).unwrap().join(","));
}
