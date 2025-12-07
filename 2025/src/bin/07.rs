use hashbrown::HashMap;

#[aoc::main(07)]
fn main(input: &str) -> (usize, usize) {
    let m = input.split('\n').map(str::as_bytes).collect::<Vec<_>>();

    let start = m[0].iter().position(|&c| c == b'S').unwrap();
    let mut beams = HashMap::from([((0, start), 1)]);
    let mut p1 = 0;
    loop {
        let mut new_beams = HashMap::new();
        for (&(r, c), &v) in &beams {
            match m.get(r+1).and_then(|row| row.get(c)) {
                Some(b'.') => {
                    *new_beams.entry((r+1, c)).or_default() += v;
                }
                Some(b'^') => {
                    for cc in [c-1, c+1] {
                        *new_beams.entry((r+1, cc)).or_default() += v;
                    }
                    p1 += 1;
                }
                None => continue,
                _ => unreachable!()
            }
        }
        if new_beams.is_empty() {
            break;
        }
        beams = new_beams;
    }
    (p1, beams.values().sum())
}
