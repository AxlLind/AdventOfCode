# Notes
This is my thoughts I wrote down after I completed each challenge.

## Day 01 - [link](./src/bin/01.rs) (72/58)
It's december again, finally! So excited to do this again this year. Thought about maybe doing it in F# this year but decided against it. Rust again it is!

Very easy problem of course since it's day 1. Servers crashed just when I was about to submit! I think I got lucky because after a while I managed to submit my answer, and I got 54th place IN THE WORLD!!!

![leaderboard](../imgs/leaderboard-2020-01.png)

Solved using a double and triple for loop. Later cleaned it up and used `tuple-combinations` from the `itertools` crate. So both a fast (`1ms`) and clean solution.

Edit: They later removed the leaderboards for day 1 since it was unfair, which makes sense. Still though!

## Day 02 - [link](./src/bin/02.rs) (710/3514)
This was one of those "find correct passwords" puzzles that have shown up previous years. The input was a bit more complex (`4-5 m: mmpth`). I chose to alter the input before using it: `"4-5 m: mmpth" -> (4,5,b'm',b"mmpth")`, i.e a tuple of `usize, usize, u8, &[u8]`, and hard-code it into my solution.

Otherwise, the problem itself was quite easy. Part one, count the occurrences, check if it's within the given range. Part two, look at the two chars and check if exactly 1 is equal to the provided char.

This day made it clear why Rust is not the best language for speed programming. I got silly borrowing compilation errors in filter for example, which were easy to fix but took some time. My biggest take away from today is that it is **always** worth it to read the input clearly! I wasted so much time by trying to save like 5 seconds and actually read the instructions. Got rate-limited after I submitted the wrong answer, because I did not read the question clearly. Will not do that again..

## Day 03 - [link](./src/bin/03.rs) (2935/2550)
You have to calculate the number of trees in a given slope. For some reason I was just quite slow on part one. I started thinking of [day 10](https://github.com/AxlLind/AdventOfCode2019/blob/master/src/bin/10.rs) of 2019, which was just a thousand times more difficult. This problem was easy, so I should have just continued.

For part one, the fact that the lines continues was easy, obviously just modulo. I chose to use a cycle iterator: `(0..W).cycle().step_by(dw)`. That might have been the only tricky part for people. After I did part one, doing part two was quite easy. In my solution I iterated over all rows since `dh == 1`. That was the only thing that was not general about my initial solution. Breaking it out into a function, general over `dw, dh` was easy otherwise.

So not my best day. I was just not that fast on this one but that's okay :)

## Day 04 - [link](./src/bin/04.rs) (3468/1688)
This was entirely a parsing challenge. When I realized that I got super nervous since Rust is not always the smoothest when it comes to string handling in my experience. However, it was surprisingly easy to do this in Rust, I mean just look at how easy it was to parse each passport:

```Rust
let passport = s.split_whitespace()
  .flat_map(|p| p.split(':'))
  .tuples()
  .collect::<HashMap<_,_>>();
```

I was a bit slow on star one since I misread how to parse the input, but got it after a while. For star two, it was just about implementing all these rules which was also surprisingly easy in Rust. The char primitive has some amazing functions to check what type of char it is, like `c.is_ascii_digit()` and `c.is_ascii_hexdigit()`, which made it quite easy.

Overall, I placed (3468/1688), so could have been faster on star one but very happy with my performance on star two!

## Day 05 - [link](./src/bin/05.rs) (13055/12372)
First weekend. I overslept since my alarm was only set for weekdays, so missed the leaderboard completely today. That's fine though! Not sure I'll make it through december, going up at 5:50 each day.

The input was just a disguised binary encoding. I choose to implement it like the instructions, with a binary search. For part two, I struggled a few minutes with understanding exactly what the instructions meant, but it wasn't too bad. Used a hashset for fast lookup, which made it `O(n)`. Another solution would have been to sort the list and check for a missing entry but that would be `O(nlogn)`.

## Day 06 - [link](./src/bin/06.rs) (1472/2632)
Today was really fun! First a bit more challenging problem in my opinion. Managed to get a respectable score on both stars today which I am really happy with. Part one I realized quickly would be solved nicely with the [itertools::unique](https://docs.rs/itertools/0.9.0/itertools/trait.Itertools.html#method.unique) function:

```Rust
s.chars()
  .filter(|c| !c.is_whitespace())
  .unique()
  .count()
```

Then you just had to sum it up. This was both quick and really easy to implement, so got an ok score on the leaderboard.

Part two was a bit more difficult. Initially, I started using HashSets, and using `set1.intersection(set2)`, ran into borrow issues and quickly switched approach. The possible answers are only `a-z`, which we can easily fit in a `u32` and let each bit represent if the answer is present or not. You then just have to `and` them together, starting with the full set `0x3ffffff`. The Rust number primitives also have an amazing function [u32::count_ones()](https://doc.rust-lang.org/std/primitive.u32.html#method.count_ones) which made this approach very easy to implement.

```Rust
s.split_whitespace()
  .map(|part| part.bytes().fold(0u32, |x, b| x | 1 << (b - b'a')))
  .fold(!0, |acc, x| acc & x)
  .count_ones()
```

Not using HashSets means no extra allocations. Both stars are solved in a single iterator expression each, without any collect. My solution is therefore quite fast, `0ms` on my machine.

## Day 07 - [link](./src/bin/07.rs) (662/335)
Best placing on the leaderboard ever for me (not counting day 1 this year), super happy with that! Rust is not really known for being a good AoC speed language, so that's fun! I guess I was just quick with realizing the recursive pattern.

This problem was really about being comfortable with recursion. There were simple patterns for both of them:

```Rust
fn contains_gold(map: &BagMap, bag: &str) -> bool {
  bag == "shiny gold" || map[bag].iter().any(|(_,b)| contains_gold(map, b))
}

fn total_bags(map: &BagMap, bag: &str) -> u32 {
  1 + map[bag].iter().map(|(c,b)| c * total_bags(map, b)).sum::<u32>()
}
```

Both of my solutions get an off-by-one error since the gold bag itself gets counted. That tripped me up on part one, but I tested with the test input and quickly realized the problem. I also heavily preprocessed the input this time. I realized it would be quite complicated to parse so I edited it by hand and embedded it in the problem directly, probably a bit faster than figuring out the parsing code.

My initial implementation (the one above) finished in 6ms, relatively fast. I later added memoization to part one, which brought it down to 0ms!

## Day 08 - [link](./src/bin/08.rs) (1320/544)
Pretty happy with my star 2 placing! Nothing to much to say about today's challenge. Just had to implement each instruction correctly. Used an array of booleans to check if we had a cycle. For part 2, I just brute forced and checked changing each acc/jmp instruction.

Is this the new VM we will be using for the rest of the challenges?? I was hoping something like the IntCoder would return this year! This one seems a bit annoying though, if the instructions will be signed. You would have to do a bunch of `as usize` everywhere.

## Day 09 - [link](./src/bin/09.rs) (2209/2417)
Was not too quick today for some reason. Struggled with stupid off-by-one errors and stuff. I think my part one ended up very clean. The stdlib function [`slice::windows()`](https://doc.rust-lang.org/std/primitive.slice.html#method.windows) is perfect for this!

```Rust
INPUT.windows(26)
  .find(|chunk| chunk[0..25].iter()
    .tuple_combinations()
    .all(|(a,b)| a + b != chunk[25])
  )
  .map(|chunk| chunk[25])
  .unwrap()
```

For my part two, I keep indexes `(i,j)` and the current sum. If the sum is less than the target I increase `j` and if it's larger I increase `i`. This works only because all numbers are positive in this case. By keeping track of the total sum and only adding/removing `arr[i]/arr[j]`, it ends up being quite fast. About `70μs` on my machine.

## Day 10 - [link](./src/bin/10.rs) (2476/1360)
Kind of slow on part one, but pretty happy with my star 2 placing! For part one, I just sorted the list, iterated over it, and looked at the difference between neighbors. Could be done in O(n) by using a HashSet instead.

```Rust
v.iter().tuple_windows()
  .fold((1,1), |(ones, threes), (a, b)| match b - a {
    1 => (ones + 1, threes),
    3 => (ones, threes + 1),
    _ => unreachable!(),
  });
```

For part two, this was the graph problem of finding the number of paths between two nodes in an acyclic graph. I initially implemented a recursive backtracking algorithm with memorization. Wasted some time, not realizing step size `2` was valid.

It can be solved more easily with a dynamic programming algorithm. The ways to get to node i are the sum of the ways to get to nodes i-1, i-2, i-3. This recursive relation makes it easy to implement the DP algorithm. They both run in about 70μs on my machine, however.

## Day 11 - [link](./src/bin/11.rs) (934/1800)
Very happy to get top 1k on star one! Messed up a bit on star two, accidentally returned when I found `.` as well. That cost me a few minutes.

This was another one of those game of life challenges, bound to be at least one every year! Just about implementing the rules correctly and making sure not to mutate the array while performing each step of the simulation. I kept two copies of the map. Read from one and wrote to the other during the simulation step, and then swapped them. Here [`std::mem::swap`](https://doc.rust-lang.org/std/mem/fn.swap.html) comes in handy to quickly swap the vectors.

I managed to reuse all of the simulation code between parts 1 and 2 by passing in a `should_swap` function since that was the only difference between the two. It also seems to not incur any runtime penalty, it is just as fast if I copy the code. The signature looks a bit ugly but still nice to not copy-paste the code.

```Rust
fn run_simulation<F: Fn(&[Vec<char>], usize, usize) -> bool>(should_swap: F) -> usize {...}
```

My solution is ok fast, around `26ms` for both stars on my machine. A bit unsure of how I would improve that.

## Day 12 - [link](./src/bin/12.rs) (241/996)
My best placing on the leaderboard ever! Super, super happy with today. Fun to be able to compete against all the Python people with a language like Rust that's not known for being a good AoC "speed" language.

The problem itself was not too bad. For part one, I think Rust pattern matching is what made me able to finish it so quickly. The rotation in part 2 was obviously the difficult part. It was not something I had in my head so I quickly googled the formulas and found [this page](https://lexique.netmath.ca/en/rotation-in-a-cartesian-plane/). That made it quite easy to implement but still took a bit for me.

Not much else to say about my solution. I just iterate over each element in the input, match on the character, and do the operation. I just love these small utility functions the stdlib has on the primitives, for example [`x.abs()`](https://doc.rust-lang.org/std/primitive.i64.html#method.abs) and [`r.rem_euclid(360)`](https://doc.rust-lang.org/std/primitive.i64.html#method.rem_euclid) to get the rotation in `[0,360)` for part one.

```Rust
match d {
  'N' => y += n,
  'S' => y -= n,
  'E' => x += n,
  'W' => x -= n,
  'L' => r -= n,
  'R' => r += n,
  'F' => match r.rem_euclid(360) {
    0   => y += n,
    90  => x += n,
    180 => y -= n,
    270 => x -= n,
    _ => unreachable!(),
  }
}
```

The problem size is very small, but finishes in `0ms`!

## Day 13 - [link](./src/bin/13.rs) (263/438)
Yesterday I got my best leaderboard position ever and today I beat it again! I suspect this is a day many, many people will struggle with and be frustrated by. It requires some relatively advanced math, [Chinese remainder theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem). You can probably solve it without it but the problem is formulated exactly as CRT.

For part one, I just brute forced it. I started at the given target and incremented it until it was divisible by one of the bus numbers.

Part two is the interesting one and definitely the most difficult so far. Initially, I started thinking about the least common multiple. Took me a bit but eventually, CRT came to mind. We had a list of different moduli and satisfying different remainders for each, that's CRT! We want to find the number `n` such that for each buss `b` with an offset `i` the following holds:

```
(n + i) mod b = 0
<=>
n mob = b - i
```

This is exactly the phrasing of a CRF problem. I just pulled an implementation from [rosetta code](https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust)! No reason to implement something as standard as CRT myself.

## Day 14 - [link](./src/bin/14.rs) (413/201)
Third day in a row where I set a new personal best on the leaderboard! I really liked today's puzzle. It required an understanding of both bitwise operations and recursion for part two (at least for me, you can probably solve it without it).

Initially, I edited the input to make my life easier. Since it followed such a simple pattern, I could do it in a few seconds using `select all occurrences` and multi cursor editing. This way I could skip writing any boring parsing code.

For part one, I kept the last mask I had seen. Before every write, I just iterated over every char in the last mask and performed the operation. You can probably do something by keeping two masks, creating them once, and simply `and/or` the value before writing. Would be a lot faster but the problem size is small so it did not matter.

For part two, I just wrote a recursive function that splits at every encounter of an `X`. Not sure how you would avoid that. No matter what you do, you will have `2^n` unique addresses to write to, where `n` is the number of X:s in the mask. In my input, the max number of X:s was 9, so just 512 addresses at most. I guess that's why it works.

Finishes in `7ms` on my machine. Part two takes up pretty much all running time. Not really sure how to make that faster.

## Day 15 - [link](./src/bin/15.rs) (760/2313)
Today was very disappointing. Worst problem so far I think. I wasted so much time on part two, thinking about ways to maybe find a cycle but realizing that would probably never happen. In the end, I just tried brute-force and surprisingly that just worked... Not a very satisfying problem at all.

Think my solution ended up very clean though. `HashMap::insert()` gives you the back element if it already existed, avoiding two hashmap lookups!

```Rust
let mut seen = [9,19,1,6,0,5].iter()
  .enumerate()
  .map(|(i,&e)| (e, i+1))
  .collect::<HashMap<_,_>>();
(7..target).fold(4, |last, i| i - seen.insert(last, i).unwrap_or(i))
```

Finishes in about `2.8 seconds` on my machine.

## Day 16 - [link](./src/bin/16.rs) (566/1977)
Very happy with part 1 but had a frustrating error on part two! Definitely one of the most "implementation heavy" so far. Took me an hour and still got top 2k.

A single misplaced `i` instead of a `j` meant the simple solution for part two did not work.. Started trying to implement a recursive backtrack approach to find a valid assignment of rules. When that did not work either I realized something else must be wrong. Those types of errors are always annoying.

For part two, I first collect which rules are possible for which indexes. I then try to find a rule with only one possible assignment. We then know that has to be the correct assignment! By then removing that as a possibility for all others, we eventually get a unique solution. This just happens to be the case with our input, that it has a unique solution.

Somewhat fast solution, about `260μs` on my machine.

## Day 17 - [link](./src/bin/17.rs) (227/407)
One of my best days ever! My solution was to keep a `HashSet<Pos>` of all active cells. Then for each round, I create a `HashMap<Pos, usize>`, a count of how many neighbors a certain position has. To do this you just have to iterate over each active cell and add it as a neighbor to all its neighbours cells!

```Rust
for (x,y,z,w) in active {
  for (dx,dy,dz,dw) in &NEIGHBOURS {
    *neighbours.entry((x+dx, y+dy, z+dz, w+dw)).or_insert(0) += 1;
  }
}
```

Then in I simply recreate the hashset of active cells from that hashmap:

```Rust
count_neighbours(&active).iter()
  .filter(|(pos,n)| match (n,active.contains(pos)) {
    (2,true) | (3,_) => true,
    _ => false,
  })
```

There was not too much difference between part one and two, so I was able to reuse the simulation code between the two by passing in the `count_neighbours` function. I created the giant list of 80 neighbors manually by hand. It's actually quite easy if you're just very systematic about it and follow the pattern.

Finishes in about `20ms` on my machine, not sure if that's fast or not. I think pretty much all time is in HashMap/HashSet operations, so if you use arrays on a fixed size grid instead it would be a lot faster probably.

## Day 18 - [link](./src/bin/18.rs) (1327/1537)
Maybe not the prettiest code I've written but pretty happy with my placing today. A more proper parser and not handling the string directly all the way through would be a lot cleaner. But my solution does zero allocations or copying of the string at least and is therefore quite fast, about `200μs`.

For part one, I just keep track of the last operator and apply it whenever you see a number/parenthesis. When you see a parenthesis I find the matching one, and recursively evaluate everything within them.

For part two, my solution was to eagerly evaluate all add operations, by looking ahead and evaluating the term as long as the next operation was an add.

## Day 19 - [link](./src/bin/19.rs) (362/931)
My solution for today is definitely not the most elegant, I'm very happy with my placing. Might rework it later. It seems many people used the [CYK algorithm](https://en.wikipedia.org/wiki/CYK_algorithm), which I had not heard of before, or leveraged regexes. I just kind of hand-rolled my own solution to this.

For part one, I created a function to get all matches of a rule and then I checked which strings in the input are in that list. This is horribly inefficient and takes over 2 seconds to run on my machine.. But it works and was quick to implement!

For part two, you really had to check your input this time. The zero rule is just `0: 8 11`, so with the given rules, you get that the string has to match 1 or more prefixing "42:s" and 1 or more matching `42 x 31`. I fetched all matches for `42` and `31` with the same function I made for part one and then I just checked if any of the input matches that pattern. I didn't use any regexes, just implemented two recursive backtracking functions to try and match rule 8 and 11.

Later, I rewrote part one, to make use of the matches on 42, 31, and checked if it matched those, without the repetitions added in part two. That made it finish in about `52ms` instead of 2 seconds!

Edit: I rewrote this using regexes. It was both cleaner and faster. [Here](https://github.com/AxlLind/AdventOfCode2020/blob/18dc6ebc94532c328833d919407ccf2058eb9a55/src/bin/19.rs) is my initial solution, though.

## Day 20 - [link](./src/bin/20.rs) (359/116)
Insane day! My highest placing on the leaderboard by far. The problem was very implementation heavy but maybe not that "algorithmically" difficult.

For part one, I assumed there would be some relatively easy way to uniquely identify the corners. I created a map of `border -> [ids with that border]` and checked for tiles that had 2 unique borders. This worked and gave only the 4 corners! Finished it quite quickly. This also revealed that at most two tiles have the same border, making the problem significantly easier.

Part two was very annoying. I finished it in about 1 hour and 15 minutes and that still got me my best placing **ever**. Since we know the borders match uniquely, we only need to check one border to know the tile that has to be placed there. I therefore started with one of the corners and placed it in the top left. I then go row by row. If it's the first tile I look at the tile above and find the other tile that matches with it's bottom border. Then for all others I do the same process but with the tile to the left. A key insight was to flip the tile if it did not exactly equal the border above/to the left. With this each tile is correctly rotated/flipped!

I think what made me finish today so quickly was just immediately getting the unique border matching part and also aiming for the right abstraction level. I created a `Tile` class to handle rotations, flips, and finding a matching tile below/to the right. This made the other code relatively easy.

## Day 21 - [link](./src/bin/21.rs) (1323/898)
Took me way too long to even understand the problem. Just stared at it for a while. Eventually, I decided to try and see if the intersection between all lists containing a certain allergen might be small. When it was only 2 for "nuts" I figured it out. Isn't this just more or less the same as day 16?

For part one, I compute the intersection of ingredients between all lists containing a certain allergen. Then it was the same process as [day 16](https://github.com/AxlLind/AdventOfCode2020/blob/master/src/bin/16.rs), whereby you eliminate possibilities by iteratively looking at what allergen only has one possible ingredient it can be. Once again, this leads to a unique solution in our input.

Part two was basically free. I was a bit unsure about how to sort by key in Rust so I initially edited and sorted it by hand in the terminal. Later I remembered that sorting a tuple in Rust works just as you expect, which means a simple [`.sorted()`](https://docs.rs/itertools/0.9.0/itertools/trait.Itertools.html#method.sorted) from the itertools crate sorts in the way we want!

Finishes in `0ms`.

## Day 22 - [link](./src/bin/22.rs) (471/281)
What a day! Super happy with my placing. Finally got to make good use of Rust's amazing [`VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html) data structure.

Not too much to say about today's solution. The problem was not too difficult. Just had to really carefully read the rules for part two. I stored each player's deck in a `VecDeque` which allows for efficient `pop_front` and `push_back`.

My solution is not very fast though, about `200ms`. Not entirely sure why and how to make it faster.

Edit: Thanks u/aceshades for the idea of storing only hashes! Saved 500ms.

## Day 23 - [link](./src/bin/23.c) (1180/560)
I usually write in **Rust** but realizing part two required linked-lists I kind of panicked and instantly switched to C. You can probably do some relatively easy solution in Rust by storing them all in an array and using indexes. In the moment, however, I figured it would be way faster to switch to C. You should always use the right tools for the right job and in C this was easy!

Kind of slow on part one, but very happy with my start two placing! Luckily I've had to use C quite a bit for a few courses recently so I was prepared. My solution just uses a singly-linked-list and does the manipulations on that list. By storing all the nodes in an array, with the node at index `i` having the value `i`, we also get O(1) look-up of a node with a particular value!

I was also able to fully reuse all code between parts one and two which is nice. Only about 60 lines of C code, shorter than many python solutions on reddit.

Finishes in about `620ms` on my machine, which is okay-ish fast I guess. Not sure how to make it faster.

Edit: I later came back and [wrote this in Rust](./src/bin/23.rs) as well. It was relatively easy if you just use a `Vec<usize>`, where the index is the node, and the value at the next is the pointer.

## Day 24 - [link](./src/bin/24.rs) (992/628)
Pretty happy with my placing today! I had heard of hexagonal grids before but never implemented them myself. I quickly found [an amazing resource](https://www.redblobgames.com/grids/hexagons/#neighbors-axial) that made implementing this quite easy! You just use a 2D-grid and figure out the neighbours in a slightly more complex way.

As for part two, that felt a bit boring. Isn't this the third time we're doing a game of life? The rules were not that complex and people should have an idea on how to handle those from previous days. The only twist is the hexagonal tiles but you had to figure that out for part one. I guess the tiles are maybe too far away to do this in an array of array grid and instead you have to use a hashmap approach, but that's my go-to anyway. Kind of an easy day 24.

Finishes in about `55ms` on my machine.

## Day 25 - [link](./src/bin/25.rs) (346/284)
Very happy with my placings today! Basically just a brute-force solution. We cannot solve the [discrete logarithm](https://www.wikiwand.com/en/Discrete_logarithm) problem so not much else you can do! You can use [modular exponentiation](https://www.wikiwand.com/en/Modular_exponentiation) to compute the final key but it really isn't needed for such small numbers. Finishes in `4ms` on my machine.

This year definitely felt _a lot_ easier than the last one. Almost no graph problems, no path finding algorithms, and no insane math problems like [day 22, 2019](https://github.com/AxlLind/AdventOfCode2019/blob/master/src/bin/22.rs). It's really hard finding the right balance. I think 2019 was maybe a bit too hard and discouraged a lot of people. I like that this year was more approachable but perhaps it was a bit too easy towards the end.

Thanks for an amazing year! I've had an alarm set for 5:50 for 25 days in a row now. Excited to not set one for tomorrow 🎄
