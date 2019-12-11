use std::time::Instant;
use itertools::Itertools;

fn main() {
  let now = Instant::now();
  let input = [[2,2,1,2,2,2,2,1,0,2,0,2,2,2,2,0,2,2,2,2,2,1,2,2,2,2,2,2,2,2,1,0,2,2,2,2,0,2,1,0,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,1,2,1,2,2,2,2,2,1,2,2,2,0,2,2,2,2,0,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,1,2,2,1,2,2,1,2,2,2,2,2,1,2,2,2,2,2,2,0,2,2,2,2,2],[2,2,2,2,2,2,2,0,1,2,0,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,2,1,2,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,1,2,1,2,2,2,2,2,1,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,1,2,2,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2],[2,2,1,2,2,2,2,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,1,2,1,0,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,1,2,2,2,0,2,2,0,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,1,1,2,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2],[2,2,1,2,2,2,2,2,2,2,1,2,2,2,2,0,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,0,2,2,2,2,1,2,1,0,2,1,2,2,2,2,2,1,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,2,2,2,0,2,2,2,2,2,2,1,2,2,2,1,2,0,2,2,2,2,2,0,2,2,2,1,2,2,2,2,1,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,0,2,2,2,2,2,1,2,2,2,2,2,2,1,2,2,2,2,2],[2,2,2,2,2,2,2,1,1,2,0,2,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,1,2,2,2,2,0,2,0,2,2,0,2,2,2,2,2,0,2,2,2,2,2,2,2,2,0,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,2,2,2,0,2,2,2,2,2,2,0,2,2,2,0,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,1,2,2,2,2,2,2,2,2,2,2,1,2,2,1,2,1,0,2,2,2,2,2,1,0,2,2,2,2,2,0,2,2,2,2,2],[2,2,1,2,2,2,2,0,1,2,0,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,0,2,2,2,2,2,1,2,0,0,2,1,2,2,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,1,2,2,2,0,2,2,2,2,0,2,2,2,1,2,2,1,2,2,2,2,2,2,2,2,2,2,1,2,2,1,2,0,0,2,2,2,2,2,0,0,2,2,2,2,2,1,2,1,2,2,2],[2,2,2,2,2,2,2,0,1,2,0,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,1,1,2,2,2,2,1,2,0,1,2,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,1,2,2,2,2,2,2,0,2,2,2,1,2,1,2,2,2,2,2,2,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,0,2,2,2,2,2,1,2,2,2,2,2,2,1,2,1,2,2,2],[2,2,0,2,2,2,2,1,1,2,0,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,1,2,2,2,2,0,2,0,1,2,2,0,2,2,2,2,0,2,2,2,2,2,2,2,2,0,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,1,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,0,2,0,2,2,1,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,0,2,2,1,2,2,2,2,2,2,2,2,1,0,2,2,2,2,2,2,2,2,2,2,2],[2,2,1,2,2,2,2,2,0,2,1,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,1,2,2,2,2,0,2,0,2,2,1,1,2,2,2,2,0,2,2,2,2,2,0,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,2,2,1,2,2,2,2,2,2,1,2,2,2,0,2,0,2,2,2,2,2,1,2,2,2,2,2,1,2,2,1,2,2,2,0,2,2,0,2,2,1,2,2,2,2,2,2,2,2,2,2,1,2,0,0,2,2,2,2,2,0,0,2,2,2,2,2,1,2,2,2,2,2],[2,2,2,2,2,1,2,2,1,2,0,0,2,2,2,1,2,2,2,2,2,2,2,0,2,2,2,2,2,2,1,0,2,2,2,2,2,2,1,0,2,1,0,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,2,2,2,2,2,2,1,2,2,2,1,2,0,2,2,2,2,2,1,2,2,2,1,2,1,2,2,2,2,2,2,1,2,2,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,0,1,2,2,2,2,2,0,2,2,2,0,2,2,1,2,0,2,2,2],[2,2,2,2,2,1,2,2,1,2,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,1,2,2,2,2,2,2,1,1,2,1,1,2,2,2,2,0,2,2,2,2,2,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,1,2,2,1,2,2,2,2,2,2,2,2,2,2,1,2,1,2,2,2,2,2,1,2,2,2,1,2,2,2,2,1,2,2,2,0,2,2,1,1,2,2,2,2,2,2,2,2,2,1,2,2,1,2,2,1,2,2,2,2,2,1,0,2,2,2,2,2,0,2,2,2,2,2],[2,2,1,2,2,0,2,2,1,2,2,2,1,2,2,1,2,2,2,2,2,0,2,0,2,2,2,2,2,2,2,0,2,2,2,2,1,2,0,0,2,0,0,2,2,2,2,0,2,2,2,2,2,1,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,0,2,2,2,2,2,2,1,2,2,2,0,2,0,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,0,0,2,2,1,2,2,1,2,0,2,2,2],[2,2,2,2,2,1,2,2,1,2,0,0,0,2,2,1,2,2,2,2,2,2,2,1,2,2,2,2,2,2,1,1,2,2,2,2,0,2,1,0,2,1,1,2,2,2,2,1,2,2,2,2,2,1,2,2,2,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,2,2,2,0,2,2,2,2,2,0,2,2,2,2,2,0,2,2,2,1,2,1,2,2,1,2,2,2,2,2,2,1,1,2,0,2,2,2,2,2,2,2,2,2,2,2,2,0,1,2,2,2,2,2,1,0,2,2,2,2,2,1,2,0,2,2,2],[2,2,0,2,2,2,2,1,2,2,1,1,2,2,2,2,2,2,2,2,2,0,2,0,2,2,2,2,2,2,0,1,0,2,2,2,1,2,0,0,2,0,1,2,2,2,2,1,2,2,2,2,2,1,2,2,0,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,2,2,0,1,2,2,2,2,2,2,2,2,1,2,2,0,1,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,2,2,0,2,2,2,0,2,2,1,2,0,2,2,2],[2,2,1,2,2,2,2,2,1,2,0,1,1,2,2,0,2,2,2,2,2,2,2,1,2,2,2,2,2,2,0,2,0,2,2,2,1,2,2,2,2,1,1,2,2,2,2,2,2,2,2,2,2,0,2,2,0,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,2,1,1,2,0,2,2,2,0,2,2,2,1,2,1,2,2,2,2,2,2,2,2,2,2,1,2,1,1,2,2,2,2,2,1,1,2,2,0,2,2,0,2,0,2,2,2],[2,2,1,2,2,2,2,1,1,2,2,1,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,2,2,2,1,2,1,2,2,2,0,2,1,0,2,2,0,2,2,2,2,1,2,2,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,0,2,2,1,2,2,2,2,2,2,1,2,2,2,0,2,0,2,2,2,2,2,0,2,2,2,1,2,0,2,2,0,2,2,2,1,2,2,1,1,2,1,1,2,2,2,2,2,2,1,2,2,0,2,2,1,2,2,2,2,2,1,2,2,2,1,2,2,0,2,2,2,2,2],[2,2,2,2,2,1,2,2,1,2,2,2,2,2,2,1,2,2,2,2,2,0,2,2,2,2,2,2,2,2,0,0,1,2,2,2,0,2,1,2,2,1,0,2,2,2,2,2,2,2,2,2,2,1,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,1,2,0,2,2,2,2,2,2,2,2,2,1,2,1,0,2,2,2,2,2,2,2,2,2,1,2,0,1,2,2,2,2,2,2,1,2,2,2,2,0,1,2,2,2,2,2,2,0,2,2,1,2,2,1,2,2,2,2,2],[2,2,2,2,2,1,2,2,2,2,0,1,0,2,2,0,2,2,2,2,2,1,2,1,2,2,2,2,2,2,2,1,2,2,2,2,1,2,2,1,2,0,2,2,2,2,2,0,2,2,2,2,2,0,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,0,2,1,2,2,2,2,2,2,2,2,2,0,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,1,2,1,2,2,0,2,2,2,0,2,2,2,2,2,0,2,2,2,2,2,2,2,0,2,2,0,2,1,2,1,2,2,2,2,0,0,2,2,1,2,2,2,2,2,2,2,2],[2,2,0,2,2,1,2,1,2,2,0,0,1,2,2,1,1,2,2,2,2,1,2,2,2,2,2,2,2,2,1,1,1,2,2,2,0,2,2,0,2,2,2,2,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,2,2,2,2,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,2,0,1,2,2,2,2,2,1,2,2,0,0,2,1,0,2,2,2,2,2,2,1,2,2,1,2,2,1,1,2,2,2,2,0,0,2,2,0,2,2,2,2,0,2,2,2],[2,2,1,2,2,2,2,0,0,2,0,2,1,2,2,1,0,2,2,2,2,2,2,0,2,2,2,2,2,2,2,1,2,2,2,2,1,2,0,0,2,0,0,2,2,2,2,0,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,0,2,2,2,2,2,2,2,2,2,2,2,0,2,2,0,2,2,2,1,2,2,0,2,2,0,0,2,2,2,2,2,2,1,2,2,0,2,2,0,1,2,2,2,2,0,1,2,2,1,2,2,1,2,1,2,2,2],[2,2,1,2,2,1,2,1,2,2,0,0,1,2,2,0,0,2,2,0,2,1,2,1,2,2,2,2,2,2,2,2,1,2,2,2,1,2,2,0,2,1,2,2,2,2,2,0,2,2,2,2,2,2,2,2,0,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,1,2,2,2,2,2,1,2,2,2,2,2,1,1,2,0,2,2,2,1,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,1,2,0,1,0,2,2,2,2,0,0,2,2,1,2,2,0,2,1,2,2,0],[2,2,1,2,2,0,2,1,1,2,2,2,0,2,2,0,2,2,2,1,2,0,2,0,2,2,2,2,2,2,0,0,0,2,2,2,2,2,1,2,2,2,0,2,2,2,2,1,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,0,2,2,1,2,2,2,2,2,2,2,2,2,2,1,2,1,2,2,2,2,2,0,2,2,2,2,2,0,1,2,1,2,2,2,0,2,2,2,1,2,0,1,2,2,2,0,2,2,0,2,2,0,2,1,0,2,2,2,2,2,2,1,2,2,0,2,2,0,2,1,2,2,1],[2,2,1,2,2,2,2,0,0,2,2,1,1,2,2,1,2,2,2,1,2,0,2,1,2,2,2,2,2,0,0,0,1,2,2,2,2,2,0,0,2,1,2,2,2,2,2,0,2,2,2,2,2,1,2,2,0,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,1,2,1,0,2,0,2,2,2,2,2,2,2,2,2,2,2,2,0,2,1,2,2,2,2,2,0,2,2,2,2,2,1,2,2,0,2,2,2,0,2,2,0,0,2,2,0,2,2,2,1,2,2,0,2,2,2,2,1,1,2,2,2,2,2,2,0,0,2,0,2,2,1,2,0,2,2,2],[2,2,1,2,2,2,2,2,0,2,0,2,1,2,2,0,2,2,2,0,2,0,2,1,2,2,2,2,2,0,0,1,1,2,2,2,0,2,1,0,2,0,0,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,0,2,0,2,1,2,2,2,2,2,2,2,2,2,1,2,2,2,1,2,1,2,2,2,2,2,2,2,2,2,1,2,1,0,2,1,2,2,2,0,2,2,0,1,2,2,0,2,2,2,2,2,2,2,2,2,1,2,0,1,1,2,2,2,2,0,0,1,2,0,2,2,2,2,0,2,2,0],[2,2,1,2,2,1,2,1,1,2,0,2,0,2,2,1,1,2,2,2,2,0,2,0,2,2,2,2,2,1,2,0,1,2,2,2,1,2,0,1,2,1,0,2,2,1,2,1,2,2,2,2,2,0,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,0,2,0,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,2,2,2,2,2,2,0,2,1,1,2,1,2,2,2,2,2,2,2,1,2,1,2,2,2,2,0,1,2,2,2,2,0,2,2,0,0,2,2,2,2,2,1,2,2,1,2,2,2,2,0,2,0,0],[2,2,0,2,2,1,2,1,1,2,1,1,0,2,2,0,0,2,2,0,2,2,2,0,2,2,2,2,2,0,2,1,2,2,2,2,1,2,0,1,2,2,2,2,0,1,2,0,2,2,2,2,2,0,2,2,1,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,1,2,2,2,2,2,2,2,2,0,2,2,1,2,2,1,2,2,2,2,0,2,2,2,2,2,0,2,0,0,2,2,2,2,2,1,2,1,2,0,2,2,1,2,1,2,2,0],[2,2,1,2,2,1,2,0,1,2,2,2,1,2,2,0,1,2,2,1,2,1,2,0,2,2,2,2,1,2,0,0,2,2,2,2,0,2,0,1,2,1,1,2,1,0,2,1,2,2,2,2,2,1,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,2,2,2,0,2,2,2,2,2,2,1,2,2,2,0,2,1,2,2,2,2,2,1,2,2,2,0,2,1,2,2,0,2,2,2,0,2,2,0,0,2,1,0,2,2,2,0,2,2,1,2,2,2,2,2,0,0,2,2,2,2,2,2,1,2,1,2,2,2,2,1,2,0,0],[2,2,0,2,2,1,2,1,0,2,2,1,1,2,2,1,0,2,2,1,2,0,2,0,2,2,2,2,1,0,2,2,0,2,2,2,0,2,0,1,2,1,0,2,0,1,2,2,2,2,2,2,2,0,2,2,0,2,2,2,1,2,2,1,2,2,2,2,2,2,2,2,2,2,1,2,0,0,2,0,2,2,2,2,2,2,0,2,2,2,1,2,1,2,2,2,2,2,1,2,2,2,2,2,0,0,2,2,2,2,2,1,2,2,1,1,2,0,1,2,2,2,0,2,2,2,2,2,1,2,0,2,0,2,2,2,2,1,1,0,2,2,2,2,2,2,0,2,2,2],[2,2,1,2,2,0,2,1,1,2,1,1,0,2,2,2,2,2,2,1,2,1,2,1,2,2,2,2,0,2,1,1,0,2,2,2,1,2,2,1,2,0,2,2,2,2,2,2,2,2,2,2,2,0,2,2,0,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,1,2,0,0,1,2,2,0,2,2,2,2,2,2,0,2,2,2,2,2,1,2,2,2,2,2,0,2,2,2,0,2,1,0,2,1,2,2,2,1,2,2,1,0,2,0,1,2,2,2,0,0,2,2,2,2,0,2,0,0,1,2,2,2,2,2,2,2,2,1,2,2,1,2,2,2,2,0],[2,2,0,2,2,2,2,0,1,2,0,1,1,2,2,2,0,2,2,1,2,1,2,2,2,2,2,2,0,2,1,0,1,2,2,2,2,2,1,0,2,2,1,2,2,1,2,1,2,2,2,2,2,1,2,2,1,2,2,2,1,2,2,1,2,2,0,2,2,2,2,2,1,2,0,0,0,0,2,2,2,2,2,2,2,0,0,2,2,2,2,2,2,2,2,2,2,2,1,2,2,2,0,2,0,0,2,0,2,2,2,1,2,1,0,1,2,1,2,2,2,2,1,1,2,0,2,2,0,2,1,2,1,2,2,2,2,1,1,0,2,1,2,2,1,2,1,2,0,2],[2,2,0,2,2,1,2,1,1,2,2,2,0,2,2,0,0,2,2,1,2,1,2,1,2,2,2,2,0,0,2,1,1,2,2,2,0,2,0,1,2,1,1,2,1,2,2,1,2,2,2,2,2,0,2,2,1,2,2,2,1,2,2,0,2,2,2,2,2,2,2,2,1,2,1,2,1,0,2,0,2,2,2,2,2,0,0,2,2,2,0,2,2,2,2,2,2,2,0,2,2,2,2,2,1,0,2,1,2,2,2,0,2,1,1,1,2,2,0,2,2,2,2,0,2,1,2,2,2,2,0,2,2,2,2,2,2,0,1,1,2,0,2,2,2,2,1,2,2,1],[2,2,0,2,2,1,2,2,1,2,0,2,0,2,2,2,2,2,2,2,2,1,2,1,2,0,2,2,0,1,2,2,2,2,2,2,0,2,2,2,2,1,1,2,1,0,2,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,1,2,0,1,2,2,2,1,2,2,2,2,2,2,1,2,2,2,1,2,0,2,2,2,2,2,1,2,2,2,2,2,2,1,2,1,2,2,2,2,2,1,2,0,2,0,0,2,2,2,2,2,2,0,2,2,0,2,2,2,0,2,2,2,2,1,0,2,2,0,2,2,2,2,1,2,1,0],[2,2,0,2,2,0,2,2,1,2,0,2,0,2,2,1,2,2,2,1,2,2,2,1,2,1,2,2,2,2,2,1,2,2,2,2,1,2,1,0,2,2,0,2,1,1,2,0,2,2,2,2,2,1,2,2,2,2,2,2,0,2,2,0,2,2,1,0,2,2,2,2,1,0,2,0,2,2,2,2,2,2,2,2,2,2,1,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,1,2,1,0,2,2,2,2,2,2,2,1,0,0,2,1,2,2,2,2,1,2,2,0,2,2,2,2,2,2,2,2,2,2,2,1,1,1,2,0,2,2,1,2,0,2,1,2],[2,2,2,2,2,2,2,0,2,2,2,0,0,2,2,1,2,2,2,1,2,2,2,0,2,1,2,2,0,2,0,2,0,2,2,2,2,2,2,1,2,2,0,2,2,1,2,1,2,2,2,2,2,2,2,2,1,2,2,2,1,2,2,0,2,2,0,2,2,2,2,2,2,0,1,2,1,1,2,2,2,2,2,2,2,1,0,2,2,2,0,2,0,2,2,2,2,2,2,2,2,2,2,2,1,0,2,2,2,2,2,1,2,0,1,1,2,2,2,2,1,2,1,2,2,2,2,2,0,2,0,2,1,2,2,2,2,2,0,1,2,1,2,1,1,2,2,2,2,2],[2,2,1,2,2,1,2,1,2,2,1,2,2,2,2,0,1,2,2,2,2,2,2,2,2,2,2,2,0,2,1,1,2,2,2,2,2,2,1,1,2,1,0,2,1,0,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,0,2,2,1,1,2,1,2,2,2,2,2,0,1,2,2,2,1,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,1,2,0,1,2,1,2,0,2,2,0,2,2,1,2,0,2,1,2,2,2,2,2,1,2,2,1,0,2,0,1,2,2,1,0],[2,2,2,2,2,1,2,0,0,2,2,1,2,2,2,0,1,2,2,0,2,2,2,0,2,1,2,2,2,0,0,1,2,2,2,2,0,2,0,2,2,1,0,2,1,2,2,0,2,2,2,2,2,1,2,2,0,2,2,2,2,2,2,1,2,2,0,0,2,2,2,2,1,2,1,0,2,1,2,2,2,2,2,2,2,1,2,2,2,2,2,2,1,2,2,2,2,2,2,2,2,2,0,2,1,2,2,0,2,2,2,2,2,2,2,0,2,2,0,2,2,2,1,0,2,2,2,2,2,2,1,1,1,2,2,2,2,2,2,1,2,0,0,1,2,0,0,2,1,0],[2,2,0,2,2,2,2,2,1,2,1,1,2,2,2,1,1,2,2,2,2,2,2,0,2,2,2,2,1,2,1,0,0,2,2,2,1,2,0,1,2,0,2,2,1,2,2,2,2,2,0,2,2,0,2,2,0,2,2,2,0,2,2,0,2,2,2,2,2,2,2,2,1,0,1,1,2,1,2,0,2,2,2,2,2,2,1,2,2,2,0,2,0,2,2,2,2,2,1,2,2,2,1,2,2,2,2,1,2,2,2,1,2,2,2,2,2,2,2,2,0,2,2,0,2,0,2,2,2,2,1,0,1,2,2,2,2,1,0,0,2,1,0,2,2,0,0,2,0,1],[2,2,2,2,2,0,2,2,0,2,2,0,0,2,2,0,0,2,2,0,2,0,2,1,2,2,2,2,0,0,1,1,1,2,2,2,0,2,1,1,2,1,2,2,0,1,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,0,0,2,2,2,2,1,0,0,0,2,2,2,1,2,2,2,2,2,1,0,2,2,2,1,2,1,2,2,2,2,2,1,2,2,2,0,2,2,2,2,0,2,2,2,2,2,0,1,1,2,1,1,2,0,2,1,0,2,1,2,2,1,2,0,2,1,2,2,2,2,0,1,2,2,2,0,1,2,0,2,2,0,1],[2,2,2,2,2,0,2,0,0,2,2,2,1,2,2,2,2,2,2,1,2,0,2,0,2,0,2,2,1,1,1,1,0,2,2,2,1,2,2,2,2,1,0,2,0,2,2,1,2,2,0,2,2,0,2,2,2,2,2,2,2,2,2,0,2,2,1,0,2,2,2,2,1,2,2,2,0,2,2,1,2,2,2,2,2,2,2,2,2,2,0,2,1,2,2,2,2,2,0,2,2,2,1,2,2,2,2,1,2,2,2,2,2,2,0,1,2,2,0,2,1,2,2,1,2,2,2,2,2,2,1,2,0,2,2,2,2,0,1,1,2,1,1,2,1,0,2,2,0,1],[2,2,1,2,2,0,2,1,2,2,1,2,0,2,2,2,2,2,2,1,2,1,2,1,2,0,2,2,2,0,0,2,2,2,2,2,0,2,2,0,2,2,1,2,0,0,2,2,2,2,1,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,1,2,0,0,2,0,2,1,2,2,2,2,2,2,0,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,1,2,1,2,0,2,2,2,2,1,2,1,0,2,2,2,1,2,0,2,0,2,2,1,2,2,0,2,2,0,2,2,2,2,2,1,1,1,2,2,1,2,2,2,2,2,2,0],[2,2,2,2,2,0,2,1,2,2,0,0,1,1,2,0,0,1,2,2,2,1,1,0,2,0,2,2,1,0,0,1,1,2,2,2,0,2,0,0,2,2,2,2,2,0,2,1,2,2,1,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,1,2,2,2,2,2,1,0,2,0,2,0,2,0,2,2,2,2,2,1,0,2,2,2,2,2,1,2,2,2,2,2,0,2,2,2,2,2,2,0,0,0,2,2,2,0,2,2,2,1,2,1,0,2,2,2,0,0,2,2,2,2,0,2,1,0,2,2,2,2,2,2,0,1,2,0,1,2,2,2,1,2,2,2],[2,2,2,2,2,0,2,1,0,2,0,0,1,1,2,1,1,1,2,0,2,0,2,2,2,0,2,2,1,0,1,0,2,2,2,2,1,2,1,2,2,1,0,2,0,0,2,0,2,2,1,2,2,0,2,2,2,2,2,2,0,2,2,1,2,2,1,0,2,2,2,2,0,0,0,2,1,2,2,1,2,2,2,2,2,1,0,2,2,2,1,2,1,2,2,2,2,2,1,2,2,2,1,2,2,1,2,0,2,2,2,2,2,0,0,2,2,2,1,2,1,2,0,0,2,1,2,2,0,2,0,1,0,2,2,2,2,2,1,0,2,2,1,2,2,1,2,2,2,0],[2,2,0,2,2,0,2,2,1,2,2,2,0,2,2,2,2,1,2,1,2,1,1,2,2,2,2,2,2,0,0,1,1,2,2,2,1,2,2,0,2,2,1,2,0,2,2,0,2,2,1,2,2,2,2,1,0,2,2,2,1,2,2,0,2,2,2,1,2,2,2,2,0,1,0,2,2,2,2,1,2,2,2,2,2,2,1,2,2,2,2,2,0,2,2,2,2,2,0,2,2,2,2,2,0,2,1,2,2,2,2,0,2,1,1,2,2,1,1,2,1,2,0,1,2,0,2,2,0,2,2,0,2,2,2,2,2,0,2,1,2,1,1,0,2,1,2,2,1,0],[2,2,2,2,2,2,2,2,0,2,1,0,1,1,2,0,1,1,2,2,2,1,1,2,2,1,2,2,0,0,0,0,2,2,2,2,2,2,2,0,2,0,0,2,2,0,2,0,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,1,2,2,0,1,2,2,2,2,0,1,0,1,2,2,2,0,2,2,2,2,2,0,1,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,1,2,1,1,0,0,2,2,2,0,2,1,0,0,2,1,1,2,2,2,0,2,2,2,2,2,0,2,1,0,0,2,2,2,2,0,1,1,2,0,1,0,1,2,1,2,0,1],[2,2,2,2,2,2,2,1,1,2,2,0,0,0,2,1,1,1,2,2,2,2,1,0,2,1,2,2,0,2,1,1,2,2,2,2,2,2,0,1,2,1,1,2,1,0,2,0,2,2,0,2,2,0,2,0,2,2,2,2,1,2,2,0,2,2,2,1,2,2,2,2,1,0,2,0,0,2,2,1,2,2,2,2,2,2,1,2,2,2,1,2,2,2,2,2,2,0,0,2,2,2,1,2,1,0,1,0,2,2,2,1,2,0,1,0,2,2,0,2,0,2,2,2,2,1,2,2,1,2,2,2,2,2,2,2,2,2,1,1,2,1,2,1,0,0,2,2,0,0],[2,2,2,2,2,1,2,2,1,2,2,0,2,2,2,0,0,0,2,2,2,0,2,0,2,1,2,2,2,1,2,1,1,2,2,2,0,2,0,0,2,2,0,2,0,0,2,1,2,2,2,2,2,0,2,2,1,2,2,2,1,2,2,2,2,2,0,2,2,2,2,2,2,2,1,2,1,1,2,0,2,2,2,2,2,0,0,2,2,2,0,2,0,2,2,2,2,0,0,2,2,2,2,2,1,1,1,0,2,2,2,0,2,0,1,0,2,0,0,2,0,2,2,0,2,2,2,2,2,2,1,0,2,2,2,2,2,2,2,2,2,1,1,1,0,0,2,2,2,0],[2,2,0,2,2,2,2,2,2,2,1,2,1,2,2,0,0,1,2,2,2,1,0,1,2,0,2,2,1,1,2,0,2,2,2,2,2,2,0,1,2,0,1,2,1,2,2,1,2,2,1,2,2,1,2,1,0,2,2,2,0,2,2,1,2,2,2,0,2,2,2,2,0,1,0,2,1,2,2,1,2,2,2,2,2,2,2,2,2,2,0,2,0,2,2,2,2,1,2,2,2,2,0,2,0,0,1,1,2,2,2,2,2,2,0,2,2,1,0,2,2,2,2,0,2,1,2,2,2,2,0,2,2,2,2,2,2,2,2,1,2,2,2,1,0,2,2,2,1,0],[2,2,2,2,2,2,2,1,0,2,2,2,0,2,2,2,2,2,2,2,2,2,0,0,2,2,2,2,2,0,0,0,0,2,2,2,0,2,1,2,2,0,1,2,0,0,2,0,2,2,2,2,2,2,2,2,1,2,2,2,0,2,2,2,2,1,0,1,2,2,2,2,2,0,2,1,0,2,2,0,2,2,2,2,2,0,2,2,2,2,1,2,0,2,2,2,2,1,2,2,2,2,1,2,1,1,1,1,2,2,2,1,2,0,0,2,2,1,2,2,1,2,0,0,2,0,2,2,1,2,0,2,0,2,2,2,2,2,1,2,2,0,0,1,1,0,2,2,2,0],[2,2,2,2,2,0,2,2,1,2,0,2,1,2,2,2,2,1,2,0,2,1,2,2,2,2,2,2,0,1,2,0,2,2,2,2,1,2,2,2,2,2,1,2,2,0,2,1,2,2,2,2,2,0,2,2,1,2,2,2,2,2,2,0,2,2,0,0,2,2,2,2,0,1,0,2,1,1,2,0,2,2,2,2,2,2,2,2,2,2,0,2,1,2,2,2,2,1,2,2,2,2,0,2,1,1,2,2,2,2,2,2,2,1,2,0,2,2,0,2,2,2,2,1,2,0,2,2,0,2,1,0,1,2,2,2,2,0,2,0,1,0,0,2,0,1,1,2,2,2],[2,2,0,2,2,1,2,1,2,2,0,2,2,2,2,1,2,0,2,0,2,1,1,2,2,0,2,2,1,1,0,0,0,2,2,2,2,2,1,1,2,0,1,2,1,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,2,1,2,1,0,0,2,2,2,2,1,0,0,2,1,0,2,0,2,2,2,2,2,1,2,2,2,2,0,2,0,2,2,2,2,2,2,2,2,2,2,2,0,0,1,0,2,2,2,2,2,2,0,2,2,0,2,2,0,2,2,0,2,1,2,2,1,2,2,2,2,2,2,2,2,0,1,2,2,0,2,0,1,0,1,2,2,2],[2,2,0,2,2,0,2,2,0,2,1,0,2,2,2,1,2,0,2,1,2,1,2,1,2,2,2,2,1,0,2,1,0,2,2,2,2,2,2,0,2,2,0,2,0,2,2,0,2,2,1,2,2,1,2,0,2,2,2,2,0,2,2,0,2,0,1,0,2,2,2,2,2,2,1,0,0,2,2,2,2,2,2,2,2,1,0,2,2,2,2,2,0,2,2,2,2,0,0,2,2,2,2,2,0,2,2,0,2,2,2,0,2,2,2,2,2,0,1,2,0,2,1,2,2,0,2,2,0,2,1,2,2,2,2,2,2,1,0,2,1,2,0,0,2,1,0,2,1,2],[2,2,2,2,2,1,2,0,1,2,0,1,2,0,2,2,2,0,2,1,2,0,2,0,2,0,2,2,1,2,2,2,1,2,2,2,2,2,1,2,2,2,1,2,0,2,2,1,2,2,2,2,2,0,2,2,1,2,2,2,1,0,2,1,2,0,2,2,2,2,2,2,2,2,1,1,1,0,2,2,2,2,2,2,2,0,2,2,2,2,1,2,1,2,2,2,2,1,2,2,2,2,1,2,1,1,0,0,2,2,2,2,2,1,1,2,2,1,1,2,0,2,2,1,2,2,2,2,2,2,2,1,2,2,2,2,2,0,1,1,1,1,2,1,0,1,0,2,1,0],[2,2,1,2,2,0,2,1,2,2,2,1,2,1,2,1,1,2,2,2,2,1,2,1,2,1,0,2,1,0,1,1,2,2,2,0,2,2,0,1,2,1,0,2,2,0,2,2,2,2,2,2,2,2,2,1,1,2,2,2,0,2,2,2,2,1,0,1,2,2,2,2,1,2,0,1,0,2,2,1,2,2,2,2,2,0,0,2,2,2,0,1,0,2,2,2,2,0,1,2,2,2,1,2,0,0,1,2,2,2,2,2,2,0,0,0,2,1,2,2,2,2,1,2,2,1,2,2,0,2,0,2,1,2,2,2,2,2,0,0,0,2,2,0,0,1,1,2,1,0],[2,2,2,2,2,0,2,0,0,2,2,2,2,0,2,1,0,0,2,0,2,0,2,0,2,1,1,2,0,2,2,2,1,2,2,1,0,2,2,2,2,2,0,2,0,2,2,1,2,2,1,2,2,0,2,0,1,2,2,2,1,0,2,0,2,1,0,2,2,2,2,2,0,0,2,1,1,1,2,0,2,2,2,2,2,0,1,2,2,2,2,2,1,2,2,2,2,2,1,2,2,2,2,2,1,1,0,2,2,2,2,1,2,2,1,0,2,1,1,2,2,2,0,0,2,1,2,2,0,2,0,2,1,2,2,2,2,1,2,2,2,2,2,0,1,0,1,2,1,1],[2,2,0,2,2,2,2,0,1,2,2,0,0,1,2,1,2,1,2,0,2,0,2,2,2,0,0,2,2,0,2,1,0,2,2,1,2,2,2,0,2,1,1,2,0,0,2,1,2,2,0,2,2,1,2,2,2,2,2,2,0,2,2,0,2,0,1,2,2,2,2,2,0,0,0,1,0,2,2,1,2,2,2,2,2,2,1,2,1,2,0,2,1,2,2,2,2,1,0,2,2,2,1,2,2,2,0,1,2,2,2,2,2,0,1,1,2,0,2,2,2,2,0,1,2,1,2,2,1,2,1,1,0,2,2,2,2,0,1,1,2,1,2,2,1,0,0,2,2,2],[2,2,0,2,2,1,2,2,1,2,1,1,2,0,2,0,2,1,2,2,2,1,2,2,2,1,0,2,2,2,1,2,0,2,2,1,1,2,2,0,2,1,1,2,1,0,2,2,2,2,2,2,2,2,2,0,2,2,2,2,1,1,2,2,2,0,1,2,2,2,2,2,0,2,0,2,2,2,2,0,2,2,2,2,2,0,2,2,1,2,1,2,2,2,2,2,2,0,0,2,2,2,1,2,0,0,1,2,2,2,2,2,2,2,2,1,2,0,2,2,1,2,1,1,2,2,2,2,0,2,2,1,0,2,2,2,2,2,1,0,0,1,0,2,2,0,2,2,1,0],[2,2,1,2,2,0,2,0,0,2,0,1,2,0,2,0,0,1,2,1,2,0,1,1,2,0,1,2,0,1,1,0,2,2,2,2,2,2,2,2,2,2,1,2,1,0,2,0,2,0,2,2,2,1,2,0,1,2,2,2,0,0,2,2,2,2,1,0,2,2,2,2,1,2,0,0,0,0,2,2,2,2,2,2,2,2,1,2,0,2,0,2,0,2,2,2,2,1,2,2,2,2,0,2,0,2,2,1,2,2,2,1,2,2,1,0,2,2,0,2,0,2,2,2,2,0,2,2,2,2,1,0,2,2,2,2,2,0,1,2,1,0,0,0,2,1,1,2,1,2],[2,2,2,2,2,2,2,2,0,2,1,1,0,0,2,0,2,2,2,2,2,1,0,1,2,0,0,2,2,2,0,0,0,2,2,2,2,2,1,1,2,1,0,2,2,1,2,0,2,1,2,2,2,2,2,1,0,2,2,2,1,0,2,1,2,1,1,0,2,2,2,2,2,1,0,2,2,0,2,2,2,2,2,2,2,0,0,2,0,2,0,0,1,2,2,2,2,2,1,2,2,2,2,2,1,0,2,2,2,2,2,1,2,2,1,0,0,1,2,2,0,2,0,2,2,1,2,2,0,0,2,0,0,2,2,2,2,2,1,2,0,2,2,2,0,2,2,2,1,0],[2,2,2,2,2,0,2,1,2,2,1,1,1,2,2,2,2,1,2,0,2,0,1,1,2,1,1,2,2,2,0,1,0,2,2,2,0,2,2,0,2,2,1,2,0,0,2,1,2,0,2,2,2,0,2,0,2,2,2,2,1,2,2,0,2,2,0,2,2,2,2,2,1,2,1,1,2,0,2,2,2,2,2,2,2,1,1,2,2,2,2,0,1,2,2,2,2,1,1,2,1,2,0,2,2,0,0,0,2,2,2,0,2,2,2,0,0,0,0,2,0,2,2,0,2,2,2,2,2,2,2,2,1,2,2,2,2,1,1,1,0,0,0,1,0,2,2,2,0,1],[2,2,1,2,2,1,2,1,2,2,2,1,1,0,2,2,2,1,2,2,2,0,2,1,2,0,1,2,1,0,0,0,2,1,2,2,0,2,1,2,2,2,2,2,0,0,2,2,2,2,2,2,2,0,2,1,0,2,2,2,0,1,2,1,2,1,1,2,2,2,2,2,2,0,1,0,1,1,2,2,2,2,2,2,2,1,1,2,2,2,0,1,1,2,2,2,2,1,0,2,2,2,2,2,1,0,1,2,2,2,2,2,2,1,0,2,0,2,2,2,1,2,1,0,2,0,2,2,0,1,2,0,2,2,2,2,2,1,2,1,0,1,2,0,1,2,1,2,1,1],[2,2,1,2,2,0,2,0,2,2,1,0,0,0,2,1,1,2,2,0,2,1,1,1,2,2,0,2,0,2,0,0,0,2,2,1,2,2,1,0,2,2,0,2,2,2,2,0,2,0,0,2,2,0,2,1,1,2,0,2,1,2,2,2,2,0,0,0,2,2,2,2,2,0,1,0,1,2,2,1,2,2,2,2,2,2,1,2,1,2,1,0,2,2,2,2,2,0,2,2,2,2,2,2,0,1,0,2,2,2,2,1,2,0,0,1,1,1,1,2,2,2,2,1,2,1,2,2,1,1,1,1,2,2,2,2,2,1,2,1,1,0,1,2,2,2,2,2,1,0],[1,2,1,2,2,0,2,2,0,2,0,1,1,2,2,1,1,2,2,1,2,2,0,0,2,1,0,2,0,1,2,1,2,1,2,1,0,2,0,0,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,0,0,2,0,2,2,0,2,2,2,2,0,2,2,0,2,2,2,0,1,0,1,1,2,2,2,2,2,2,2,1,1,2,1,2,2,1,2,2,2,1,2,2,1,2,0,2,0,2,1,0,1,0,2,2,2,1,2,2,1,2,0,2,0,2,0,2,1,1,2,2,2,2,1,1,0,0,1,2,2,2,2,1,1,2,0,0,2,2,1,0,1,2,0,1],[0,2,2,2,2,1,2,0,0,2,0,1,0,0,2,1,1,0,2,1,2,1,1,0,2,0,2,2,2,2,1,0,2,2,2,0,1,2,1,2,2,2,1,2,0,0,2,2,2,2,2,2,2,2,2,2,1,2,1,2,1,2,2,1,2,1,0,2,2,2,2,2,1,2,2,0,1,0,2,0,2,2,2,2,2,1,2,2,0,2,1,1,2,2,2,2,2,0,0,2,0,2,1,2,0,1,1,0,2,2,2,2,2,1,0,2,0,0,2,2,0,2,0,0,2,0,2,2,0,1,2,0,0,2,2,2,2,1,1,2,2,1,0,1,1,1,0,2,2,0],[1,2,1,2,2,2,2,2,1,2,0,0,0,2,2,1,0,2,2,1,2,2,2,1,2,2,0,2,0,1,0,2,0,2,2,1,2,2,2,1,2,0,2,2,0,2,2,1,2,0,0,2,2,1,2,2,2,2,0,2,2,0,2,2,2,0,2,1,2,1,2,2,1,2,0,1,0,2,2,1,2,2,2,2,2,0,0,2,2,2,0,2,2,2,2,0,2,0,1,2,1,2,1,2,0,1,1,2,2,2,2,1,2,0,2,0,2,1,1,2,2,2,1,1,2,0,2,2,0,2,0,2,2,2,2,2,2,0,1,2,2,1,0,2,0,1,1,2,0,1],[2,2,2,2,2,2,2,0,2,2,1,2,1,2,0,1,1,1,2,1,2,2,2,0,2,0,1,2,0,0,2,1,1,1,2,2,2,0,0,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,0,1,1,1,2,0,2,2,1,2,0,2,2,2,0,2,2,0,0,0,0,0,1,2,2,2,2,2,2,2,0,1,2,2,2,0,0,2,2,2,2,2,2,0,2,2,2,0,2,2,1,2,1,2,2,2,0,2,1,1,1,0,2,2,2,2,2,0,1,2,1,2,2,0,0,1,0,0,2,2,2,2,0,1,1,2,2,0,0,2,0,0,2,1,2],[0,2,1,2,2,2,2,2,0,2,0,0,1,1,2,1,1,2,2,2,2,0,2,2,2,2,0,2,2,1,0,1,1,2,2,2,2,0,0,0,2,1,1,2,2,2,2,2,2,0,1,2,2,1,2,1,2,2,1,2,0,2,2,0,2,2,0,1,2,2,2,2,2,0,0,0,2,2,2,1,2,2,2,2,2,2,2,2,2,2,2,1,1,2,2,0,2,1,2,2,1,2,2,2,2,2,1,0,2,2,2,0,2,1,2,2,2,0,0,2,0,2,0,0,2,0,2,2,1,1,1,0,0,2,2,2,2,1,1,1,1,2,0,2,2,0,0,2,0,1],[0,2,0,2,2,0,2,2,2,2,2,2,2,0,1,0,1,1,2,1,2,0,1,1,2,0,1,2,1,2,2,0,2,2,2,0,0,2,0,2,2,2,2,2,1,0,2,1,2,0,1,2,2,1,2,1,1,0,2,2,0,1,2,2,2,0,1,0,2,1,2,2,2,2,2,2,1,2,2,1,2,2,2,2,2,0,0,2,1,2,2,1,0,2,2,0,2,0,2,2,0,2,2,2,0,1,0,1,2,2,2,1,2,1,1,0,2,0,1,2,0,2,0,1,2,1,2,2,0,1,2,1,2,2,2,2,2,1,0,0,0,1,2,1,0,2,2,2,0,2],[1,2,2,2,2,0,2,2,1,2,0,2,2,1,2,2,0,2,2,0,2,2,0,1,2,2,0,2,2,2,0,2,2,0,2,0,2,2,1,0,2,1,0,2,2,1,2,1,2,2,2,2,2,1,2,1,1,1,2,2,1,1,2,2,2,1,2,1,2,0,2,2,1,0,0,0,0,2,2,1,2,2,2,2,2,0,1,2,0,2,1,2,1,2,2,2,2,0,2,2,2,2,2,2,1,0,0,1,2,2,2,0,2,1,1,1,1,2,2,2,2,2,1,0,2,1,2,2,0,2,1,1,2,2,2,2,2,2,2,2,2,2,0,1,1,1,1,2,2,1],[2,2,0,2,2,1,2,2,1,2,0,1,1,1,2,0,1,0,2,2,2,0,2,1,2,1,0,2,2,2,1,2,2,2,2,1,1,1,1,2,2,1,0,2,0,0,2,0,2,1,1,2,2,1,2,1,2,0,1,2,2,1,2,1,2,1,0,1,2,1,2,2,0,1,0,0,2,0,2,2,2,2,2,2,2,1,1,2,2,2,1,1,0,2,2,0,2,2,0,2,1,2,1,2,1,1,2,2,2,2,2,0,2,0,2,0,2,2,2,2,1,2,2,1,2,0,2,2,1,2,2,0,2,2,2,2,2,1,1,1,0,1,1,2,1,2,0,2,2,0],[0,2,0,2,2,1,2,2,2,2,0,0,2,0,1,0,1,0,2,1,2,0,1,1,2,0,0,2,0,1,1,0,0,2,2,0,0,1,0,2,2,1,2,2,0,2,2,0,2,1,1,2,2,2,2,2,0,2,0,2,1,1,2,0,2,2,2,2,2,0,2,2,0,2,0,0,1,2,2,1,2,2,2,2,2,1,1,2,2,2,1,0,0,2,2,2,2,0,0,2,0,0,2,2,1,0,0,0,2,2,2,2,2,1,1,0,2,2,1,2,2,2,1,0,2,1,2,2,1,1,1,2,2,2,2,2,1,0,2,2,0,2,0,2,1,0,2,2,0,1],[2,2,1,2,2,0,2,1,1,2,1,2,0,1,0,0,1,1,2,1,2,1,2,2,2,0,1,2,2,1,0,1,2,1,2,0,0,2,2,1,2,0,1,2,2,2,2,2,2,1,0,2,2,0,2,2,1,2,1,2,0,2,2,1,2,1,0,2,2,2,2,2,0,0,1,2,2,0,2,1,2,2,2,2,2,0,2,2,0,2,1,1,2,2,2,0,2,2,2,2,1,2,0,2,2,0,2,1,2,2,2,0,2,1,2,1,0,0,2,2,2,2,1,0,2,0,2,2,2,1,0,1,2,2,2,2,1,2,2,2,0,0,1,2,2,0,1,2,1,0],[2,2,0,2,2,1,2,0,1,2,2,2,1,2,0,2,1,0,2,0,2,2,0,0,2,1,2,2,0,0,1,0,1,1,2,1,2,0,0,2,2,0,1,2,2,2,2,1,2,0,1,2,2,0,2,2,2,2,0,2,2,1,2,1,2,1,1,0,2,2,2,2,2,2,2,1,2,0,2,0,2,2,2,2,2,2,2,2,0,2,2,1,2,2,2,1,2,2,0,2,0,0,0,2,2,2,0,1,2,2,2,1,2,0,2,0,2,1,0,2,0,2,2,0,2,0,2,2,1,2,0,1,2,2,2,2,1,2,2,0,0,0,1,0,0,1,1,2,1,2],[2,2,0,2,2,2,2,0,0,2,2,1,0,0,2,2,1,2,2,2,2,1,1,0,2,2,1,2,1,1,1,1,0,1,2,2,0,1,0,2,2,2,1,2,0,2,2,2,2,0,0,0,2,0,2,1,2,1,2,2,1,1,2,1,2,2,0,0,2,2,2,2,0,2,0,1,1,2,2,0,2,2,2,2,2,2,0,2,2,2,2,0,0,2,2,2,2,1,1,2,2,2,1,2,1,1,1,0,2,2,2,2,2,2,0,1,1,2,0,2,2,2,2,0,2,2,2,2,0,2,0,2,1,2,2,2,0,1,0,1,2,1,2,2,0,0,0,2,2,2],[1,2,0,2,2,1,2,2,0,2,1,2,1,2,0,2,0,2,2,0,2,0,2,1,2,1,2,2,2,1,0,1,0,1,2,1,0,0,0,2,2,1,0,2,2,2,2,1,2,2,0,0,2,0,2,0,2,2,1,2,0,1,2,1,2,2,1,1,2,1,2,2,0,0,0,2,2,0,2,1,2,2,2,2,2,1,1,2,2,2,0,1,0,2,2,0,2,2,2,2,2,0,1,2,1,2,2,0,2,2,2,1,2,0,0,2,0,2,2,2,1,2,1,0,2,1,2,2,0,0,2,0,1,2,2,2,0,0,1,1,0,0,2,1,2,0,2,2,2,1],[1,2,1,2,2,1,2,0,2,2,1,1,1,0,1,0,2,1,2,2,2,1,1,0,2,1,0,2,1,0,0,2,0,1,2,2,0,0,0,0,2,2,1,2,0,2,2,1,2,1,0,2,2,1,2,0,2,2,1,2,0,1,2,1,2,1,0,1,2,2,2,2,2,1,1,2,1,1,2,1,2,2,2,2,2,2,0,2,2,2,0,1,2,2,2,2,2,1,0,2,1,0,1,2,2,2,1,1,2,2,2,2,2,2,1,2,2,0,2,2,1,0,1,2,0,1,2,2,1,2,0,1,0,2,2,2,1,2,0,2,1,0,0,1,1,2,2,1,2,0],[2,2,0,2,2,0,2,0,2,2,0,1,0,0,1,1,1,1,2,1,2,2,2,1,2,1,2,2,2,1,2,2,1,0,2,2,0,1,1,2,2,2,1,2,2,0,2,1,2,1,0,2,2,2,2,0,0,2,2,2,2,0,2,1,2,2,1,0,2,2,2,2,0,0,2,2,1,0,2,0,2,2,2,2,2,1,0,2,0,2,2,1,0,2,2,0,2,1,1,2,2,2,1,2,0,1,2,0,2,2,2,2,2,0,1,0,0,0,1,2,1,2,2,2,1,0,2,2,0,1,1,1,0,0,2,2,1,0,1,0,0,2,0,0,1,1,1,2,1,1],[2,2,1,2,2,1,2,0,0,2,0,1,0,0,2,0,0,1,2,0,2,0,1,2,2,2,1,2,0,1,1,1,2,1,2,1,1,1,0,1,2,1,1,2,1,2,2,2,2,2,0,1,2,0,2,0,0,0,0,2,0,2,2,0,2,0,2,0,2,0,2,2,0,2,1,0,2,1,2,0,2,2,2,2,2,2,2,2,2,2,1,2,1,2,2,1,2,2,1,2,2,1,1,2,0,1,1,2,2,2,2,1,2,1,0,2,1,2,1,2,2,2,1,0,1,1,2,2,1,1,1,1,1,2,2,0,2,1,1,1,1,0,2,0,0,2,0,0,1,2],[0,2,1,2,2,0,2,0,1,2,1,0,0,0,1,1,2,0,2,0,2,1,2,0,2,2,0,2,2,1,2,1,0,1,2,1,1,0,1,0,2,0,2,2,1,1,2,2,2,1,0,1,2,2,2,0,2,0,2,2,0,2,2,2,2,1,1,1,2,2,2,2,1,1,2,2,2,1,2,0,2,2,2,2,2,1,1,2,0,2,1,1,2,2,2,0,2,2,1,2,1,1,1,2,2,1,0,2,2,2,2,2,2,1,2,2,0,2,2,2,1,1,1,1,2,2,2,2,1,2,1,2,2,0,2,2,0,1,2,2,2,2,2,2,1,2,2,2,2,0],[1,2,1,2,2,1,2,2,2,2,1,1,0,2,2,2,0,1,2,2,2,2,0,2,2,1,2,2,2,0,0,2,1,2,0,2,1,2,2,1,2,0,0,2,1,0,2,0,2,1,1,2,2,2,2,1,2,2,0,2,0,0,2,0,2,2,0,2,2,1,2,2,0,0,1,2,1,1,2,2,2,2,2,2,2,0,2,2,2,2,1,1,1,2,2,1,2,0,0,2,0,0,0,2,2,1,0,1,2,2,2,2,2,2,1,0,0,1,1,2,2,2,0,0,2,0,2,2,1,1,0,2,2,2,2,1,1,1,0,1,0,2,0,2,0,0,2,0,2,0],[0,2,0,2,2,2,2,1,0,2,2,2,1,1,0,0,1,2,2,2,2,2,0,0,2,0,1,2,1,2,1,0,2,1,0,0,0,1,0,2,2,2,0,2,0,0,2,1,2,0,0,2,2,2,2,0,0,0,2,2,2,0,2,2,2,2,0,0,2,2,2,2,1,2,2,0,2,2,2,0,2,2,2,2,2,2,1,2,2,2,1,0,2,2,2,2,2,2,0,2,0,2,2,2,1,1,0,2,2,2,2,0,0,0,1,0,0,0,2,2,2,0,1,1,1,2,2,2,0,0,0,2,1,1,2,2,0,0,2,0,0,0,2,0,0,1,2,0,2,1],[0,2,2,2,2,1,2,2,2,2,2,0,0,2,2,1,0,2,2,1,2,0,0,2,2,0,2,2,1,0,0,2,1,2,1,2,2,1,0,2,2,1,1,2,0,2,2,2,2,1,1,0,2,2,2,1,2,1,0,2,0,1,2,0,2,1,2,1,2,2,2,2,2,2,1,1,1,0,2,0,2,2,2,2,2,1,2,2,0,2,0,1,2,2,2,1,0,2,1,2,2,1,1,2,1,1,0,2,2,2,2,1,0,2,2,0,0,2,2,2,1,2,0,0,1,2,2,2,2,2,2,0,2,0,2,0,2,1,0,1,0,1,2,1,2,0,2,2,1,2],[0,2,2,2,2,2,2,0,0,2,0,0,1,0,2,2,2,2,2,1,2,2,2,0,2,1,1,2,1,1,1,0,2,0,0,0,2,1,0,1,2,0,1,2,0,2,2,0,2,2,0,0,2,2,2,0,1,2,2,2,1,0,2,0,2,0,0,0,2,2,2,2,0,2,0,0,2,0,2,2,2,2,2,2,2,1,2,2,0,2,2,0,2,1,2,1,2,1,1,2,2,0,2,2,1,2,0,2,2,2,2,1,2,2,0,1,0,0,1,2,0,0,1,1,1,2,2,2,0,0,2,2,0,1,2,1,2,1,2,2,1,1,2,2,2,1,0,2,0,2],[0,2,1,2,2,1,2,1,1,2,0,1,2,0,0,2,0,2,2,0,2,0,2,2,2,1,2,2,0,1,2,2,0,0,1,1,1,0,2,1,2,2,2,2,1,2,2,1,2,1,0,1,2,0,2,2,0,1,0,2,2,0,2,2,2,1,1,1,2,0,2,1,2,2,2,2,1,2,2,0,2,2,2,2,2,0,1,2,1,2,0,0,0,1,2,2,2,0,0,2,0,2,0,2,1,2,2,0,2,2,2,2,2,2,2,1,1,1,2,2,2,0,0,2,1,1,2,2,0,1,0,1,0,0,2,2,1,2,0,0,2,2,2,1,2,0,0,2,2,1],[0,2,2,2,2,2,2,2,1,2,0,2,1,2,0,2,1,0,2,2,2,2,1,2,2,0,1,2,1,2,1,2,2,2,0,2,2,1,2,2,2,0,2,2,0,2,2,2,2,2,2,0,2,1,2,1,0,2,2,2,0,0,2,2,2,2,1,0,2,2,2,2,0,2,0,0,0,1,2,1,2,2,2,2,2,2,2,2,0,2,1,1,2,1,2,1,0,1,2,2,2,2,2,2,1,1,1,1,2,2,2,2,2,0,1,1,0,0,1,2,1,0,0,1,1,1,2,2,0,0,2,2,2,2,2,0,0,2,0,0,0,0,2,0,0,2,1,2,2,0],[0,2,2,1,2,2,2,2,1,2,0,2,0,2,1,0,1,2,2,0,2,0,0,1,2,0,0,2,1,1,1,0,2,1,2,1,2,1,1,1,2,0,0,2,0,0,2,2,2,0,0,0,2,0,2,0,2,1,2,2,2,1,2,1,2,1,0,0,2,0,2,2,2,0,2,0,2,1,2,0,2,2,2,2,2,1,0,2,0,2,1,0,1,0,2,2,0,2,1,2,2,0,1,2,1,1,2,2,2,2,2,2,2,1,0,1,2,2,1,2,2,2,2,0,0,0,2,2,2,1,1,0,1,1,2,1,2,2,0,0,1,0,2,0,2,2,0,1,0,0],[2,2,1,1,2,0,2,0,2,2,1,2,0,0,1,0,0,1,2,0,2,1,2,1,2,2,1,2,2,2,1,2,2,2,2,2,2,2,0,0,2,2,2,2,1,1,2,0,2,1,0,1,2,1,2,0,1,1,2,2,0,0,2,2,2,2,2,0,2,0,2,0,1,1,2,1,1,0,2,1,2,2,2,2,2,0,1,2,1,2,0,2,2,0,2,2,2,0,0,2,2,1,2,2,0,1,0,2,2,2,2,2,0,2,2,1,0,1,2,2,1,2,2,0,1,0,2,2,0,2,2,0,1,2,2,1,1,0,0,2,1,2,0,1,0,1,2,0,0,2],[0,2,0,2,2,2,2,2,2,2,2,0,0,0,2,0,2,1,2,2,2,1,0,1,2,0,0,2,1,0,2,1,2,0,2,1,1,2,2,0,2,0,1,2,0,1,2,2,2,2,0,2,2,2,2,1,0,0,0,2,1,2,2,2,2,2,0,0,2,2,2,1,1,1,1,0,1,2,2,0,2,2,2,2,2,2,2,2,2,2,2,1,2,1,2,2,2,0,2,2,1,1,0,2,2,1,0,0,2,2,2,1,0,1,2,1,0,2,1,0,0,0,2,0,1,2,2,2,2,2,0,1,1,2,2,0,1,0,1,2,0,0,0,1,2,1,2,0,0,1],[2,2,0,2,2,2,2,1,0,2,1,1,0,1,0,1,1,2,2,0,2,0,0,2,2,1,2,2,2,0,1,0,1,2,0,0,1,1,0,1,2,0,2,2,1,0,2,2,2,2,0,0,2,2,2,1,0,1,0,2,2,1,2,1,2,1,0,1,2,2,1,0,0,0,0,2,0,0,2,2,2,2,2,2,2,0,0,2,1,2,0,0,0,2,2,2,2,0,1,2,0,2,2,2,2,0,1,2,2,2,2,0,2,0,0,0,1,0,0,1,0,1,0,2,2,0,2,2,2,1,1,2,0,0,2,1,2,2,2,1,0,0,2,1,0,2,0,1,1,1],[0,2,1,1,2,2,2,0,2,2,0,1,2,0,0,1,2,1,2,1,2,2,1,1,2,2,1,2,1,0,0,0,1,1,2,2,1,2,0,0,2,1,0,2,0,1,2,2,2,2,2,0,2,1,2,1,2,1,1,2,1,0,2,0,2,2,2,2,2,0,0,1,1,0,1,0,2,0,2,2,1,2,2,2,2,0,0,2,2,2,0,0,0,1,2,0,1,1,2,2,0,2,1,2,2,0,1,1,2,2,2,2,1,1,0,1,0,0,2,1,1,2,0,1,2,0,2,2,2,2,1,2,2,1,2,0,0,0,1,2,1,1,2,0,1,1,2,2,0,0],[1,2,2,0,2,2,2,2,2,2,0,1,2,1,1,0,0,0,2,0,2,1,2,0,2,2,2,2,2,1,0,2,1,1,0,2,2,2,2,2,2,1,2,2,1,1,2,1,2,2,2,1,2,0,2,2,1,1,0,2,0,0,2,0,2,0,2,1,1,2,2,0,0,0,1,2,1,2,2,0,2,2,2,2,2,0,2,2,0,2,1,0,1,0,2,1,1,2,0,2,1,1,0,2,1,0,2,1,2,2,2,0,1,2,2,1,2,2,2,2,0,2,2,2,1,0,2,2,2,0,0,1,2,1,2,1,1,1,2,0,0,2,0,0,2,0,0,1,2,0],[0,2,2,0,2,0,2,1,1,2,1,0,2,2,2,0,0,2,2,2,2,2,0,2,2,0,1,2,1,0,2,1,0,0,0,0,0,1,1,1,2,2,1,2,0,0,2,1,2,1,1,1,2,0,2,1,2,1,2,2,1,0,2,2,2,0,1,0,1,1,1,2,0,2,1,2,0,0,2,1,0,2,2,2,0,0,2,2,1,2,2,1,1,1,2,2,2,0,1,2,1,1,1,2,0,1,0,0,2,2,2,0,2,2,1,0,1,0,1,0,0,1,1,0,0,0,2,2,0,2,1,0,1,2,2,2,1,0,2,0,1,1,0,0,2,2,1,0,1,1],[2,2,2,0,2,2,2,2,2,2,0,0,1,2,2,0,0,2,2,0,2,0,1,2,2,0,2,2,2,1,1,2,2,0,1,2,0,1,0,1,2,2,2,2,2,2,2,1,2,1,0,1,2,1,2,2,1,2,1,2,0,2,2,2,2,1,0,1,1,0,2,2,2,0,1,0,2,1,2,2,1,2,2,2,1,2,1,2,2,2,1,0,0,1,2,2,2,0,0,2,1,0,0,2,2,1,0,0,2,2,2,2,0,2,0,0,2,0,1,1,2,1,0,0,2,1,2,2,0,2,1,2,2,0,0,0,0,0,2,0,1,0,1,0,2,0,0,1,2,2],[2,2,0,0,0,1,2,2,2,2,2,2,0,2,0,2,1,1,2,0,2,1,1,1,2,0,0,2,2,2,0,1,0,0,1,1,2,1,1,1,2,1,0,2,2,2,2,1,2,2,2,2,2,2,2,0,1,0,1,2,1,0,2,2,2,2,2,2,0,1,0,2,0,2,2,2,0,0,2,2,2,2,2,2,1,2,0,2,0,2,0,2,0,1,2,1,1,1,1,2,2,0,0,2,2,2,1,2,2,2,2,2,2,1,1,2,0,1,0,1,1,2,0,0,0,1,2,2,2,1,1,2,1,1,0,1,2,0,2,2,2,0,1,1,0,1,2,2,2,2],[0,2,2,1,1,0,2,0,2,2,2,2,2,1,1,2,1,1,2,1,2,2,2,2,0,2,2,2,0,1,0,2,2,0,1,2,1,2,0,2,2,0,1,2,1,0,2,1,2,0,2,1,2,1,2,1,0,2,1,2,2,2,2,1,2,0,1,2,2,2,0,0,0,2,1,2,0,1,2,1,2,2,2,2,2,1,1,2,2,2,1,1,2,1,2,1,2,2,1,2,0,1,1,2,1,2,0,0,2,2,2,1,2,0,1,1,0,2,2,2,2,0,2,2,0,0,2,2,1,0,2,1,2,2,0,1,0,1,1,2,0,0,1,2,0,0,2,2,1,0],[1,2,0,1,0,0,2,1,1,2,2,2,0,2,0,2,2,0,2,0,2,1,2,1,2,2,0,2,1,0,0,2,1,2,1,2,0,0,2,1,2,2,1,2,0,2,2,0,2,2,0,0,2,2,2,2,2,2,2,2,2,1,2,0,2,2,1,0,0,2,1,2,2,2,1,2,2,2,2,2,0,2,2,2,2,1,2,2,1,2,0,2,2,0,2,2,2,2,0,2,2,1,1,2,0,1,2,2,2,2,2,1,1,0,0,2,1,2,1,2,2,2,0,2,0,2,2,2,1,0,1,2,1,2,0,2,0,2,0,1,1,0,0,0,2,2,0,2,1,1],[1,2,2,2,1,1,2,0,2,2,0,2,2,0,1,2,1,2,2,1,2,1,2,1,2,0,2,2,0,0,1,0,1,0,1,2,1,2,2,2,2,0,2,2,0,1,2,1,1,1,1,2,2,0,2,0,2,2,2,2,1,2,2,0,2,0,0,2,2,2,2,0,1,0,1,0,1,2,2,1,1,2,2,2,0,1,2,2,0,2,1,2,2,1,2,1,0,2,2,0,0,0,0,2,2,1,2,1,2,2,2,2,1,0,0,0,2,1,2,0,1,1,0,1,2,0,2,2,0,0,1,0,1,2,1,1,2,0,2,0,2,1,1,1,0,1,2,2,1,1],[1,2,2,0,1,1,2,2,0,2,2,2,0,1,0,1,2,0,2,0,2,2,2,1,2,0,2,0,1,0,2,2,2,1,0,2,2,0,0,0,2,0,1,2,0,1,2,1,0,1,2,1,2,0,2,1,2,2,0,2,1,1,2,0,2,0,2,2,0,0,1,1,0,2,1,2,0,1,2,2,0,2,2,2,0,1,2,2,2,2,2,1,2,1,2,0,0,2,2,2,0,2,2,2,1,0,1,2,2,2,2,0,2,2,0,2,1,0,1,0,2,0,1,2,2,2,2,2,1,0,2,1,0,0,2,0,2,0,1,0,1,0,2,0,1,1,1,0,0,2],[1,2,0,2,1,0,2,1,0,2,1,2,2,0,1,0,2,1,2,0,2,1,0,0,2,2,2,1,0,0,2,2,1,2,2,2,2,1,0,1,2,0,1,2,0,1,2,0,2,2,0,0,2,0,2,0,2,2,1,2,2,0,2,0,2,0,0,0,1,1,1,1,0,1,2,2,1,2,2,2,1,2,2,2,2,0,0,2,2,0,2,0,1,2,2,0,2,2,0,0,0,2,0,2,0,1,1,1,2,2,2,1,0,1,0,0,1,1,0,1,2,2,1,2,0,0,2,2,1,2,1,0,1,2,0,2,1,1,2,0,0,2,1,0,0,2,0,1,2,1],[1,2,1,1,0,0,2,1,2,2,2,2,1,1,2,1,0,1,2,0,2,1,1,2,2,2,2,1,0,2,1,0,0,0,0,1,0,1,0,2,2,0,2,2,0,1,2,1,2,0,2,0,2,1,2,1,0,0,2,2,1,0,2,0,2,0,2,0,1,2,1,1,2,0,0,0,0,1,2,2,0,2,2,2,0,1,0,2,1,0,1,1,1,0,2,0,0,0,0,0,2,0,0,2,0,0,1,1,2,2,2,0,0,2,2,2,0,2,2,0,2,2,2,2,2,2,2,2,2,1,1,0,1,2,2,2,2,2,1,2,0,0,1,2,0,2,1,2,0,1],[2,1,1,0,1,0,1,0,2,0,2,2,2,0,2,0,1,0,0,0,1,1,1,1,1,1,0,2,2,2,1,1,1,0,2,1,0,2,2,1,1,1,2,1,2,2,0,2,2,1,0,2,1,0,0,2,0,0,2,0,1,2,0,1,0,1,2,2,1,2,0,0,0,1,1,1,2,1,0,2,0,0,0,1,2,2,1,0,1,0,2,0,0,0,0,2,1,0,0,1,2,0,0,0,0,2,0,1,1,0,1,2,2,2,1,0,1,1,2,0,0,2,1,0,2,2,1,1,0,2,2,0,1,1,0,0,1,0,0,0,2,0,2,1,2,0,2,2,2,0]];
  let image = input.iter()
    .fold(vec![2;150], |image, c| c.iter()
      .zip(image.iter())
      .map(|(a,b)| if *b == 2 { *a } else { *b })
      .collect::<Vec<_>>()
    );
  image.iter()
    .map(|i| if *i == 0 { ' ' } else { '#' })
    .chunks(25)
    .into_iter()
    .for_each(|c| println!("{}", c.collect::<String>()));
  println!("Time: {}ms", now.elapsed().as_millis());
}
