use itertools::Itertools;

static INPUT: &str = "[[[3,[8,6]],[6,1]],[[[1,1],2],[[1,0],0]]]\n[[[1,[7,3]],1],9]\n[[[2,6],[[3,1],[0,9]]],[[7,[4,8]],[[2,7],3]]]\n[[[3,[0,4]],[[8,4],[1,9]]],[7,[2,[5,7]]]]\n[[[4,5],[[0,7],1]],[9,[0,4]]]\n[[5,[[1,5],[3,6]]],8]\n[[3,[[9,3],9]],9]\n[2,[[[2,1],[0,5]],[9,9]]]\n[[2,[6,9]],[[[4,1],0],[3,4]]]\n[[[[6,8],0],[[8,8],9]],[[[4,2],3],[3,[7,3]]]]\n[[3,7],9]\n[[[[2,5],8],[2,5]],[[0,[5,7]],[[2,5],4]]]\n[[[8,[6,6]],0],[4,[[5,6],[8,4]]]]\n[[[1,[8,2]],[[0,4],[2,6]]],[[3,4],0]]\n[[1,[[9,2],[6,0]]],[[[0,9],5],[[8,0],[1,5]]]]\n[[2,[[2,3],[1,8]]],[3,[[7,2],[0,7]]]]\n[[5,4],5]\n[[[[4,2],[4,8]],[7,3]],[0,[[8,9],6]]]\n[[[6,7],0],5]\n[[2,[[9,0],[8,4]]],[[[7,4],[3,4]],0]]\n[[[9,[8,9]],1],[[5,[6,7]],3]]\n[[2,[0,0]],[3,[[2,5],[1,4]]]]\n[[0,1],[0,[[8,8],[8,3]]]]\n[[[0,2],[2,8]],[1,[[7,0],0]]]\n[[[[5,4],3],[[7,5],[2,6]]],[[5,8],[0,1]]]\n[0,[0,0]]\n[[5,[[5,6],0]],[[[2,7],9],[7,9]]]\n[[[[0,8],2],[[2,5],[7,6]]],[[9,7],[[8,7],[9,2]]]]\n[[[0,[4,6]],[[6,3],[4,4]]],[8,[[4,8],[4,8]]]]\n[[[[8,9],[3,8]],8],[[[7,9],6],[9,[2,7]]]]\n[[[[8,9],[1,6]],0],[[[8,7],4],[9,[1,4]]]]\n[5,7]\n[[[[1,5],[3,6]],[[5,5],4]],[[3,3],[4,[4,0]]]]\n[[[0,6],[5,[5,3]]],[[4,[0,0]],8]]\n[7,[6,8]]\n[[[[8,5],9],[[3,2],7]],[[[6,6],5],2]]\n[[[[4,4],[0,4]],9],0]\n[[0,[3,[9,3]]],[9,[[8,0],[0,9]]]]\n[[[[4,0],0],[1,[1,7]]],[[3,[3,0]],[[1,3],6]]]\n[[9,4],[3,[[7,1],6]]]\n[[[[3,7],7],1],[[4,3],[[6,9],[6,9]]]]\n[[[8,[2,5]],[[8,4],4]],[[[3,4],[6,7]],[5,[8,5]]]]\n[2,[4,[[3,2],7]]]\n[[[[3,1],[5,6]],[[2,7],7]],[4,[8,[7,4]]]]\n[[7,8],[[[3,9],7],2]]\n[[[[8,8],[5,8]],[[1,0],[6,0]]],[[[1,2],6],[[4,2],[5,5]]]]\n[[1,[0,9]],[[[2,1],1],1]]\n[[6,[8,1]],[4,[[7,8],5]]]\n[[[1,[1,6]],[1,[5,7]]],[[[2,8],6],0]]\n[9,1]\n[[[0,[6,5]],[[8,5],2]],[[[2,4],[7,3]],[[1,5],[9,2]]]]\n[[[2,7],[0,[3,6]]],[[[1,0],[9,6]],[1,[0,4]]]]\n[6,[[[5,9],8],[0,2]]]\n[7,[[[9,4],[8,6]],[[1,1],1]]]\n[[[2,1],0],8]\n[1,[[6,[1,4]],[[0,0],[1,9]]]]\n[[[1,[7,9]],2],8]\n[[[[0,9],2],[[8,4],9]],[0,[[7,7],[4,8]]]]\n[[1,[2,[1,8]]],[[[3,6],[2,1]],[3,[5,0]]]]\n[[3,3],[3,5]]\n[[[[9,3],[4,3]],[5,[8,1]]],[[6,[5,0]],9]]\n[0,[[9,[3,5]],3]]\n[[[9,1],0],[[[5,9],[8,0]],[7,[4,8]]]]\n[[[[7,7],8],3],[[[6,6],[6,5]],[6,4]]]\n[[[[3,7],1],[9,[4,2]]],[[9,[2,5]],[[9,0],5]]]\n[5,[[0,2],6]]\n[[[[2,7],[5,3]],[1,8]],2]\n[[[8,[7,7]],[9,[0,0]]],4]\n[[[4,[1,4]],0],[[[8,7],8],[[4,1],7]]]\n[[[[0,6],0],[[3,2],[9,8]]],[[9,[4,5]],[[7,7],[0,8]]]]\n[[[[6,3],3],[[1,5],7]],[[0,1],[7,7]]]\n[[[[2,0],2],[3,[3,5]]],[[[0,8],[8,2]],[[0,6],5]]]\n[[[6,[5,3]],[[5,5],9]],[[5,9],[[8,7],[3,7]]]]\n[[[[1,7],[3,4]],[9,2]],1]\n[[[[8,2],6],1],[[5,[2,7]],[3,9]]]\n[5,[5,7]]\n[[[[9,8],[3,4]],[[2,5],[5,6]]],[[[2,7],7],[9,[8,7]]]]\n[[[1,4],[[6,1],[1,3]]],[1,[7,[1,7]]]]\n[[[[1,4],8],[[5,1],8]],[[[1,3],[6,9]],[6,[3,3]]]]\n[[[[4,0],[0,7]],[4,5]],[4,2]]\n[3,8]\n[7,[[[7,6],5],[[6,6],5]]]\n[[[5,[0,5]],[4,4]],[3,[[4,2],[7,0]]]]\n[[[[7,9],8],[9,6]],[5,0]]\n[[[[3,0],[5,2]],1],[[[6,9],[5,3]],[[2,5],[6,3]]]]\n[7,[[[7,7],[4,5]],[9,2]]]\n[[7,[[4,2],[9,3]]],[7,[6,1]]]\n[7,9]\n[[[8,[8,1]],[[7,3],1]],[[9,8],[2,[8,3]]]]\n[[[9,3],3],3]\n[[[8,[5,7]],[[2,1],[1,3]]],[[[3,5],2],0]]\n[[[8,8],0],[[1,4],[[8,6],9]]]\n[[9,[3,[3,0]]],[1,7]]\n[1,[[[8,8],1],[2,[0,5]]]]\n[[0,[1,5]],[9,[0,[9,0]]]]\n[1,[[[1,1],[8,3]],[1,8]]]\n[[5,[[7,7],[3,3]]],[[[6,6],[7,8]],[1,[0,0]]]]\n[[[[6,7],1],[0,2]],[[[4,2],[7,6]],[[8,4],[4,9]]]]\n[[6,[[3,3],[9,0]]],[1,[[4,5],4]]]\n[[[[3,4],7],[9,0]],[[[4,5],1],[[5,1],[9,3]]]]";

fn parse_num(s: &str) -> Vec<(u32,u8)> {
  let (mut d, mut num) = (0, Vec::new());
  for c in s.chars() {
    match c {
      '[' => d += 1,
      ']' => d -= 1,
      ',' => {}
      _   => num.push(((c as u8 - b'0') as u32, d)),
    }
  }
  num
}

fn explode(num: &mut Vec<(u32,u8)>) {
  loop {
    let maxd = *num.iter().map(|(_,d)| d).max().unwrap();
    if maxd < 5 { return }
    let i = (0..num.len()).find(|&i| num[i].1 == maxd).unwrap();
    let ((a,d),b) = (num[i], num.remove(i+1).0);
    if i != 0 { num[i-1].0 += a; }
    if i+1 != num.len() { num[i+1].0 += b; }
    num[i] = (0,d-1);
  }
}

fn split(num: &mut Vec<(u32,u8)>) -> bool {
  let i = match (0..num.len()).find(|&i| num[i].0 > 9) {
    Some(i) => i,
    None => return false,
  };
  let (x,d) = num[i];
  num[i] = (x/2, d+1);
  num.insert(i+1, ((x+1)/2, d+1));
  true
}

fn add(n1: &Vec<(u32,u8)>, n2: &Vec<(u32,u8)>) -> Vec<(u32,u8)> {
  let mut num = n1.iter().chain(n2).map(|&(n,d)| (n,d+1)).collect();
  loop {
    explode(&mut num);
    if !split(&mut num) { break }
  }
  num
}

fn magnitude(mut num: Vec<(u32,u8)>) -> u32 {
  while num.len() > 1 {
    let maxd = *num.iter().map(|(_,d)| d).max().unwrap();
    let i = (0..num.len()).filter(|&i| num[i].1 == maxd).min().unwrap();
    let ((n1,d1),(n2,d2)) = (num[i], num[i+1]);
    if d1 == d2 {
      num[i] = (n1*3 + n2*2, d1-1);
      num.remove(i+1);
    }
  }
  num[0].0
}

aoc2021::main! {
  let nums = INPUT.lines().map(parse_num).collect::<Vec<_>>();
  let p1 = magnitude(nums[1..].iter().fold(nums[0].clone(), |n1,n2| add(&n1,n2)));
  let p2 = nums.iter()
    .tuple_combinations()
    .flat_map(|(n1,n2)| [magnitude(add(n1,n2)), magnitude(add(n2,n1))])
    .max()
    .unwrap();
  (p1,p2)
}
