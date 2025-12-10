use std::collections::VecDeque;
use hashbrown::HashSet;

fn parse_list(s: &str) -> Vec<usize> {
    s[1..s.len()-1].split(',').map(|w| w.parse().unwrap()).collect()
}

type Machine = (String, Vec<Vec<usize>>, Vec<usize>);

fn solve_p1((lights, buttons, _): &Machine) -> usize {
    let goal = lights.bytes().map(|c| c == b'#').collect::<Vec<_>>();
    let start_state = vec![false; lights.len()];
    let mut q = VecDeque::new();
    q.push_back((start_state.clone(), 0usize));

    let mut seen = HashSet::new();
    while let Some((state, n)) = q.pop_front() {
        if seen.contains(&state) {
            continue;
        }
        if state == goal {
            return n;
        }
        seen.insert(state.clone());

        for button in buttons {
            let mut next = state.clone();
            for &i in button {
                next[i] = !next[i];
            }
            q.push_back((next, n + 1));
        }
    }

    unreachable!("Goal must be reachable per problem statement");

}

fn solve_p2((_, buttons, jolts): &Machine) -> usize {
    use good_lp::*;
    let num_vectors = buttons.len();
    let dim = jolts.len();


    // Create integer decision variables x[0], x[1], ..., x[num_vectors-1]
    let mut vars = variables!();
    for i in 0..num_vectors {
        vars.add(variable().min(0).integer().name(format!("x{}", i)));
    }
    let press_vars = vars.iter_variables_with_def().map(|(v, _)| v).collect::<Vec<_>>();

    // Objective: minimize sum of x_i
    let problem = vars.minimise(
        (0..num_vectors).fold(0.into_expression(), |acc, i| acc + press_vars[i])
    );
    let mut problem = highs(problem);

    // For each dimension k, enforce exact sum:
    //
    //   sum_i (v_i[k] * x_i) == target[k]
    //
    for k in 0..dim {
        let mut expr = 0.into_expression();
        for i in 0..buttons.len() {
            if buttons[i].contains(&k) {
                expr = expr + press_vars[i];
            }
        }
        problem.add_constraint(expr.eq(jolts[k] as f64));
    }

    let sol = problem.solve().unwrap();
    press_vars.iter().map(|&v| sol.value(v)).sum::<f64>() as _
}

#[aoc::main(10)]
fn main(input: &str) -> (usize, usize) {
    let machines = input.split('\n').map(|l| {
        let (lights_str, rest) = l.split_once(' ').unwrap();
        let (buttons_str, jolts_str) = rest.rsplit_once(' ').unwrap();
        let lights = lights_str[1..lights_str.len()-1].to_string();
        let buttons = buttons_str.split(' ').map(parse_list).collect::<Vec<_>>();
        let jolts = parse_list(jolts_str);
        (lights, buttons, jolts)
    }).collect::<Vec<Machine>>();
    let p1 = machines.iter().map(solve_p1).sum();
    let p2 = machines.iter().map(solve_p2).sum();
    (p1, p2)
}
