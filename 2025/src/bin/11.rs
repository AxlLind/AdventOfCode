use hashbrown::HashMap;

fn count_paths<'a>(
    cache: &mut HashMap<(&'a str, bool, bool), usize>,
    g: &HashMap<&'a str, Vec<&'a str>>,
    node: &'a str,
    mut dac: bool,
    mut fft: bool,
) -> usize {
    if node == "out" {
        return if dac && fft {1} else {0};
    }
    dac |= node == "dac";
    fft |= node == "fft";
    let key = (node, dac, fft);
    if !cache.contains_key(&key) {
        let res = g[node].iter().map(|n| count_paths(cache, g, n, dac, fft)).sum();
        cache.insert((node, dac, fft), res);
    }
    cache[&key]
}

#[aoc::main(11)]
fn main(input: &str) -> (usize, usize) {
    let mut g = HashMap::new();
    for l in input.split('\n') {
        let (n, rest) = l.split_once(": ").unwrap();
        let nodes = rest.split(' ').collect::<Vec<_>>();
        g.insert(n, nodes);
    }
    let mut cache = HashMap::new();
    let p1 = count_paths(&mut cache, &g, "you", true, true);
    let p2 = count_paths(&mut cache, &g, "svr", false, false);
    (p1, p2)
}
