fn invalid(s: &[u8], k: usize) -> bool {
    if s.len() % k != 0 {
        return false;
    }
    (k..s.len()).step_by(k).all(|x| s[x..x+k] == s[0..k])
}

#[aoc::main(02)]
fn main(input: &str) -> (usize, usize) {
    let (mut p1, mut p2) = (0, 0);
    for l in input.split(',') {
        let (a, b) = l.split_once('-')
            .map(|(a,b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
            .unwrap();
        for i in a..=b {
            let s = i.to_string();
            if s.len() % 2 == 0 && invalid(s.as_bytes(), s.len() / 2) {
                p1 += i
            }
            if (1..=s.len() / 2).any(|k| invalid(s.as_bytes(), k)) {
                p2 += i;
            }
        }
    }
    (p1, p2)
}
