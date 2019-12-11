use std::time::Instant;
use std::collections::HashSet;
use itertools::Itertools;
use num_integer::gcd;

static INPUT: [&str; 33] = [
  "..#..###....#####....###........#",
  ".##.##...#.#.......#......##....#",
  "#..#..##.#..###...##....#......##",
  "..####...#..##...####.#.......#.#",
  "...#.#.....##...#.####.#.###.#..#",
  "#..#..##.#.#.####.#.###.#.##.....",
  "#.##...##.....##.#......#.....##.",
  ".#..##.##.#..#....#...#...#...##.",
  ".#..#.....###.#..##.###.##.......",
  ".##...#..#####.#.#......####.....",
  "..##.#.#.#.###..#...#.#..##.#....",
  ".....#....#....##.####....#......",
  ".#..##.#.........#..#......###..#",
  "#.##....#.#..#.#....#.###...#....",
  ".##...##..#.#.#...###..#.#.#..###",
  ".#..##..##...##...#.#.#...#..#.#.",
  ".#..#..##.##...###.##.#......#...",
  "...#.....###.....#....#..#....#..",
  ".#...###..#......#.##.#...#.####.",
  "....#.##...##.#...#........#.#...",
  "..#.##....#..#.......##.##.....#.",
  ".#.#....###.#.#.#.#.#............",
  "#....####.##....#..###.##.#.#..#.",
  "......##....#.#.#...#...#..#.....",
  "...#.#..####.##.#.........###..##",
  ".......#....#.##.......#.#.###...",
  "...#..#.#.........#...###......#.",
  ".#.##.#.#.#.#........#.#.##..#...",
  ".......#.##.#...........#..#.#...",
  ".####....##..#..##.#.##.##..##...",
  ".#.#..###.#..#...#....#.###.#..#.",
  "............#...#...#.......#.#..",
  ".........###.#.....#..##..#.##...",
];
static H: i64 = INPUT.len() as i64;
static W: i64 = INPUT[0].len() as i64;

fn map_to_astroid_coords() -> HashSet<(i64,i64)> {
  INPUT.iter()
    .enumerate()
    .flat_map(|(j,s)| s.chars()
      .enumerate()
      .filter(|&(_,c)| c != '.')
      .map(|(i,_)| (i as i64, j as i64))
      .collect_vec()
    )
    .collect()
}

fn all_unique_lines() -> Vec<(i64,i64)> {
  let (x_max, y_max) = (W-1,H-1);
  (-x_max..x_max)
    .cartesian_product(-y_max..y_max)
    .filter(|&(x,y)| gcd(x,y) == 1)
    .collect()
}

fn until_hit(
  asteroids: &HashSet<(i64,i64)>,
  (x,y): (i64,i64),
  (dx,dy): (i64,i64),
) -> Option<(i64,i64)> {
  let (mut new_x, mut new_y) = (x,y);
  while (0..H).contains(&new_x) && (0..W).contains(&new_y) {
    new_x += dx;
    new_y += dy;
    if asteroids.contains(&(new_x, new_y)) {
      return Some((new_x, new_y));
    }
  }
  None
}

fn main() {
  let now = Instant::now();
  let asteroids = map_to_astroid_coords();
  let lines = all_unique_lines();
  let answer = asteroids.iter()
    .map(|&asteroid| lines.iter()
      .filter_map(|&slope| until_hit(&asteroids, asteroid, slope))
      .count()
    )
    .max();
  println!("{}", answer.unwrap());
  println!("Time: {}ms", now.elapsed().as_millis());
}
