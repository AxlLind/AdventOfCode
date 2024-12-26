fn fits(l: &[&[u8]], k: &[&[u8]]) -> bool {
    for r in 0..l.len() {
        for c in 0..l[0].len() {
            if l[r][c] == b'#' && k[r][c] == b'#' {
                return false;
            }
        }
    }
    true
}

#[aoc::main(25)]
fn main(input: &str) -> (usize, char) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for s in input.split("\n\n") {
        let x = s.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();
        if x[0][0] == b'#' {
            locks.push(x);
        } else {
            keys.push(x);
        }
    }
    let mut p1 = 0;
    for l in &locks {
        for k in &keys {
            p1 += fits(&l, &k) as usize;
        }
    }
    (p1, '🎄')
}
