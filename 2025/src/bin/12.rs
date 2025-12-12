#[aoc::main(12)]
fn main(input: &str) -> (usize, char) {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let sizes = parts[0..parts.len()-1].iter()
        .map(|p| p.bytes().filter(|&b| b == b'#').count())
        .collect::<Vec<_>>();
    let mut p1 = 0;
    for region in parts[parts.len()-1].split('\n') {
        let (x, rest) = region.split_once(": ").unwrap();
        let (a, b) = x.split_once('x').unwrap();
        let (r, c) = (a.parse::<f64>().unwrap(), b.parse::<f64>().unwrap());
        let total_needed = rest.split(' ').zip(&sizes)
            .map(|(n, size)| n.parse::<usize>().unwrap() * size)
            .sum::<usize>();
        if total_needed as f64 * 1.3 < r * c {
            p1 += 1;
        }
    }
    (p1, '🎄')
}
