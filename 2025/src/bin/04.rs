fn round(m: &mut [Vec<u8>], remove: bool) -> usize {
    let dd = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];
    let mut res = 0;
    for r in 0..m.len() {
        for c in 0..m[0].len() {
            if m[r][c] != b'@' {
                continue;
            }
            let n = dd.iter().filter(|&&(dr, dc)| {
                let rr = (r as i64 + dr) as usize;
                let cc = (c as i64 + dc) as usize;
                m.get(rr).and_then(|row| row.get(cc)).is_some_and(|&x| x == b'@')
            }).count();
            if n < 4 {
                if remove {
                    m[r][c] = b'.';
                }
                res += 1;
            }
        }
    }
    res
}

#[aoc::main(04)]
fn main(input: &str) -> (usize, usize) {
    let (mut p1, mut p2) = (0, 0);
    let mut m = input.split('\n').map(|l| l.as_bytes().to_vec()).collect::<Vec<_>>();
    p1 += round(&mut m, false);

    loop {
        let n = round(&mut m, true);
        if n == 0 {
            break;
        }
        p2 += n;
    }
    (p1, p2)
}
