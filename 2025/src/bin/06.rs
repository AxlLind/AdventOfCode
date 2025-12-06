fn ans(problems: &[Vec<usize>], ops: &[u8]) -> usize {
    problems.iter().zip(ops)
        .map(|(p, o)| match o {
            b'*' => p.iter().product::<usize>(),
            b'+' => p.iter().sum::<usize>(),
            _ => unreachable!()
        })
        .sum()
}

#[aoc::main(06)]
fn main(input: &str) -> (usize, usize) {
    let lines = input.split('\n').collect::<Vec<_>>();
    let ops = lines[lines.len()-1].bytes().filter(|&c| c != b' ').collect::<Vec<_>>();

    let mut p1 = Vec::new();
    for l in &lines[0..lines.len()-1] {
        let ws = l.split_ascii_whitespace().collect::<Vec<_>>();
        for i in 0..ws.len() {
            if p1.len() <= i {
                p1.push(Vec::new());
            }
            p1[i].push(ws[i].parse().unwrap())
        }
    }

    let mut p2 = Vec::from([Vec::new()]);
    for c in 0..lines[0].len() {
        let w = (0..lines.len()-1)
            .map(|r| lines[r].as_bytes()[c] as char)
            .filter(|&c| c.is_ascii_digit())
            .collect::<String>();
        if !w.is_empty() {
            p2.last_mut().unwrap().push(w.parse().unwrap());
        } else {
            p2.push(Vec::new());
        }
    }

    (ans(&p1, &ops), ans(&p2, &ops))
}
