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
        .filter(|c| c.starts_with('t') && conns(c, &graph).len() > 1)
    {
        let peers = conns(c, &graph);
        for c1 in &peers {
            let peers2 = conns(c1, &graph);
            peers.intersection(&peers2).for_each(|c2| {
                let mut v = vec![c, *c1, *c2];
                v.sort();
                set.insert(v);
            });
        }
    }
    println!("{}", set.len());
}
