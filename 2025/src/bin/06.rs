fn ans(problems: &[Vec<usize>], ops: &[u8]) -> usize {
    let mut ans = 0;
    for (p, o) in problems.iter().zip(ops) {
        match o {
            b'*' => {
                ans += p.iter().product::<usize>();
            }
            b'+' => {
                ans += p.iter().sum::<usize>();
            }
            _ => unreachable!()
        }
    }
    ans
}

#[aoc::main(06)]
fn main(input: &str) -> (usize, usize) {
    let m = input.split('\n').collect::<Vec<_>>();
    let ops = m.last().unwrap().bytes().filter(|&c| c != b' ').collect::<Vec<_>>();

    let mut problems1: Vec<Vec<usize>> = Vec::new();
    for l in &m[0..m.len()-1] {
        let ws = l.split_ascii_whitespace().collect::<Vec<_>>();
        if problems1.is_empty() {
            for _ in 0..ws.len() {
                problems1.push(Vec::new());
            }
        }
        for i in 0..ws.len() {
            problems1[i].push(ws[i].parse().unwrap())
        }
    }

    let mut problems2: Vec<Vec<usize>> = Vec::new();
    problems2.push(Vec::new());
    for c in 0..m[0].len() {
        let w = (0..m.len()-1)
            .map(|r| m[r].as_bytes()[c] as char)
            .filter(|&c| "0123456789".contains(c))
            .collect::<String>();
        if !w.is_empty() {
            problems2.last_mut().unwrap().push(w.parse().unwrap());
        } else {
            problems2.push(Vec::new());
        }
    }
    let p1 = ans(&problems1, &ops);
    let p2 = ans(&problems2, &ops);
    (p1, p2)
}
