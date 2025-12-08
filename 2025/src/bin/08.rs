use itertools::Itertools;

type Point = (usize, usize, usize);

fn dist((x1, y1, z1): Point, (x2, y2, z2): Point) -> usize {
    // we dont have to sqrt for sorting to still work
    (x2 - x1).pow(2) + (y2 - y1).pow(2) + (z2 - z1).pow(2)
}

struct DisjointSet {
    parent: Vec<usize>,
    sizes: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            sizes: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra != rb {
            if self.sizes[ra] < self.sizes[rb] {
                self.parent[ra] = rb;
                self.sizes[rb] += self.sizes[ra];
            } else {
                self.parent[rb] = ra;
                self.sizes[ra] += self.sizes[rb];
            }
        }
    }

    fn comp_size(&mut self, a: usize) -> usize {
        let x = self.find(a);
        self.sizes[x]
    }
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

    let mut ds = DisjointSet::new(xs.len());
    let (mut p1, mut p2) = (0, 0);
    for i in 0.. {
        let (a, b) = dist_pairs.pop().unwrap();
        ds.union(a, b);
        if i == 1000 {
            let mut components = ds.sizes.clone();
            components.sort();
            p1 = components[components.len() - 3..].iter().product();
        }
        if ds.comp_size(a) == xs.len() {
            p2 = xs[a].0 * xs[b].0;
            break;
        }
    }
    (p1, p2)
}
