use hashbrown::HashMap;



#[aoc::main(07)]
fn main(input: &str) -> (usize, usize) {
    let m = input.split('\n').map(str::as_bytes).collect::<Vec<_>>();

    let mut p1 = 0;
    let mut beams = HashMap::new();
    beams.insert((0, m[0].iter().position(|&c| c == b'S').unwrap()), 1);
    loop {
        let mut new_beams = HashMap::new();
        for (&(r, c), &v) in &beams {
            if r+1 >= m.len() {
                continue;
            }
            match m[r+1][c] {
                b'.' => {
                    *new_beams.entry((r+1, c)).or_default() += v;
                }
                b'^' => {
                    for cc in [c-1, c+1] {
                        *new_beams.entry((r+1, cc)).or_default() += v;
                    }
                    p1 += 1;
                }
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
