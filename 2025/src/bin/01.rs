#[aoc::main(01)]
fn main(input: &str) -> (usize, usize) {
    let (mut r, mut p1, mut p2) = (50, 0, 0);
    for l in input.split('\n') {
        let x = l[1..].parse::<i64>().unwrap();
        let d = if l.as_bytes()[0] == b'R' {1} else {-1};
        for _ in 0..x {
            r += d;
            if r % 100 == 0 {
                p2 += 1;
            }
        }
        if r % 100 == 0 {
            p1 += 1;
        }
    }
    (p1, p2)
}
