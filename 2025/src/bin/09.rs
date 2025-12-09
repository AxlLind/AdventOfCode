use itertools::Itertools;

fn is_point_on_line(px: isize, py: isize, x1: isize, y1: isize, x2: isize, y2: isize) -> bool {
    let cross = (x2 - x1) * (py - y1) - (y2 - y1) * (px - x1);
    if cross != 0 {
        return false;
    }
    (px >= x1.min(x2) && px <= x1.max(x2)) &&
    (py >= y1.min(y2) && py <= y1.max(y2))
}

fn is_point_inside(px: isize, py: isize, poly: &[(isize, isize)]) -> bool {
    let mut inside = false;
    for i in 0..poly.len() {
        let (x1, y1) = poly[i];
        let (x2, y2) = poly[(i + 1) % poly.len()];
        if is_point_on_line(px, py, x1, y1, x2, y2) {
            return true;
        }
        if (y1 > py) != (y2 > py) {
            let dy = y2 - y1;
            if dy != 0 {
                let x_intersect = (x2 - x1) * (py - y1) / dy + x1;
                if px < x_intersect {
                    inside = !inside;
                }
            }
        }
    }
    inside
}

fn get_orient(ax: isize, ay: isize, bx: isize, by: isize, cx: isize, cy: isize) -> isize {
    let v = (bx - ax) * (cy - ay) - (by - ay) * (cx - ax);
    v.signum()
}

fn do_lines_intersect(a1: (isize, isize), a2: (isize, isize), b1: (isize, isize), b2: (isize, isize)) -> bool {
    let o1 = get_orient(a1.0, a1.1, a2.0, a2.1, b1.0, b1.1);
    let o2 = get_orient(a1.0, a1.1, a2.0, a2.1, b2.0, b2.1);
    let o3 = get_orient(b1.0, b1.1, b2.0, b2.1, a1.0, a1.1);
    let o4 = get_orient(b1.0, b1.1, b2.0, b2.1, a2.0, a2.1);
    o1 * o2 < 0 && o3 * o4 < 0
}

fn is_rect_inside(x1: isize, x2: isize, y1: isize, y2: isize, points: &[(isize, isize)]) -> bool {
    let corners = [(x1, y1), (x1, y2), (x2, y1), (x2, y2)];
    if !corners.iter().all(|&(cx, cy)| is_point_inside(cx, cy, points)) {
        return false;
    }

    let rect_edges = [
        ((x1, y1), (x2, y1)),
        ((x2, y1), (x2, y2)),
        ((x2, y2), (x1, y2)),
        ((x1, y2), (x1, y1)),
    ];
    points.iter()
        .circular_tuple_windows()
        .all(|(&p1, &p2)|
            rect_edges.iter().all(|&(start, end)|! do_lines_intersect(start, end, p1, p2))
        )

}

#[aoc::main(09)]
fn main(input: &str) -> (isize, isize) {
    let (mut p1, mut p2) = (0, 0);
    let points = input.split('\n').map(|l| {
        let (a, b) = l.split_once(',').unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    }).collect::<Vec<(isize, isize)>>();

    for (&a, &b) in points.iter().tuple_combinations() {
        let x1 = a.0.min(b.0);
        let x2 = a.0.max(b.0);
        let y1 = a.1.min(b.1);
        let y2 = a.1.max(b.1);
        let area = (x2 - x1 + 1) * (y2 - y1 + 1);
        p1 = p1.max(area);
        if is_rect_inside(x1, x2, y1, y2, &points) {
            p2 = p2.max(area);
        }
    }
    (p1, p2)
}
