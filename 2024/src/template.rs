

#[aoc::main($DAY)]
fn main(input: &str) -> (usize, usize) {
    let (mut p1, mut p2) = (0, 0);
    let xs = input.split('\n').map(|l| {
        l.split(',').collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    (p1, p2)
}
