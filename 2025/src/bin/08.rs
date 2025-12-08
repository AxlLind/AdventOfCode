use std::collections::VecDeque;

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

type Point = (usize, usize, usize);

fn dist((x1, y1, z1): Point, (x2, y2, z2): Point) -> usize {
    // we dont have to sqrt for sorting to still work
    (x2 - x1).pow(2) + (y2 - y1).pow(2) + (z2 - z1).pow(2)
}

fn find_components(g: &HashMap<usize, HashSet<usize>>) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut components = Vec::new();
    for &n in g.keys() {
        if visited.contains(&n) {
            continue;
        }
        let mut q = VecDeque::from([n]);
        visited.insert(n);
        let mut comp = 0;
        while let Some(c) = q.pop_front() {
            comp += 1;
            for &n in &g[&c] {
                if !visited.contains(&n) {
                    visited.insert(n);
                    q.push_back(n);
                }
            }
        }
        components.push(comp);
    }
    components
}

#[aoc::main(08)]
fn main(input: &str) -> (usize, usize) {
    let xs = input.split('\n').map(|l| {
        let (x, rest) = l.split_once(',').unwrap();
        let (y, z) = rest.split_once(',').unwrap();
        (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
    }).collect::<Vec<Point>>();
    let mut dist_pairs = (0..xs.len()).tuple_combinations().collect::<Vec<(usize, usize)>>();
    dist_pairs.sort_by_key(|&(i, j)| dist(xs[i], xs[j]));
    dist_pairs.reverse();
    let mut g = HashMap::<usize, HashSet<usize>>::new();
    let (mut p1, mut p2) = (0, 0);
    for i in 0.. {
        let (a, b) = dist_pairs.pop().unwrap();
        g.entry(a).or_default().insert(b);
        g.entry(b).or_default().insert(a);

        let mut components = find_components(&g);
        if i == 1000 {
            components.sort();
            p1 = components[components.len() - 3..].iter().product();
        }
        if components[0] == xs.len() {
            p2 = xs[a].0 * xs[b].0;
            break;
        }
    }
    (p1, p2)
}
