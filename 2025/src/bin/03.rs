fn max_batteries(xs: &[u8], l: usize) -> usize {
    let mut r = String::new();
    let mut j = 0;
    for i in 0..l {
        j = (j..xs.len() - l + i + 1).max_by_key(|&x| (xs[x], usize::MAX - x)).unwrap();
        r.push(xs[j] as char);
        j += 1;
    }
    r.parse().unwrap()
}

#[aoc::main(03)]
fn main(input: &str) -> (usize, usize) {
    let (mut p1, mut p2) = (0, 0);
    for l in input.split('\n') {
        p1 += max_batteries(l.as_bytes(), 2);
        p2 += max_batteries(l.as_bytes(), 12);
    }
    (p1, p2)
}
