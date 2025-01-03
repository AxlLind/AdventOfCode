use hashbrown::{HashMap, HashSet};

#[aoc::main(05)]
fn main(input: &str) -> (usize, usize) {
    let (s1, s2) = input.split_once("\n\n").unwrap();
    let mut orderings = HashMap::<usize, HashSet<usize>>::new();
    for l in s1.lines() {
        let (x, y) = l.split_once('|').unwrap();
        orderings.entry(y.parse().unwrap()).or_default().insert(x.parse().unwrap());
    }
    let pages = s2.lines().map(|l| {
        l.split(',').map(|w| w.parse::<usize>().unwrap()).collect::<Vec<_>>()
    });

    let (mut p1, mut p2) = (0, 0);
    for mut p in pages {
        if p.is_sorted_by(|a, b| orderings[b].contains(a)) {
            p1 += p[p.len() / 2];
        } else {
            p.sort_by(|a, b| orderings[b].contains(a).cmp(&true));
            p2 += p[p.len() / 2];
        }
    }
    (p1, p2)
}
