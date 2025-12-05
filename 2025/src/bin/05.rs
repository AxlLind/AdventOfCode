#[aoc::main(05)]
fn main(input: &str) -> (usize, usize) {
    let (s1, s2) = input.split_once("\n\n").unwrap();
    let mut ranges = s1.split('\n')
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect::<Vec<(usize, usize)>>();
    ranges.sort();
    let mut merged = Vec::from([ranges[0]]);
    for &(a, b) in &ranges[1..] {
        let &(a2, b2) = merged.last().unwrap();
        if a > b2 {
            merged.push((a, b));
        } else {
            *merged.last_mut().unwrap() = (a2, b2.max(b));
        }
    }

    let p1 = s2.split('\n').filter(|l| {
        let x = l.parse().unwrap();
        ranges.iter().any(|&(a, b)| (a..=b).contains(&x))
    }).count();
    let p2 = merged.iter().map(|&(a, b)| b - a + 1).sum();
    (p1, p2)
}
