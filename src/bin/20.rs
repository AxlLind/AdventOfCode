use std::time::Instant;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

static INPUT: [&str; 129] = [
  "                                               A U       K     C   I         L   Q           W                                         ",
  "                                               V H       F     D   F         Z   S           E                                         ",
  "  #############################################.#.#######.#####.###.#########.###.###########.#######################################  ",
  "  #...#.#.........#.#.......#...#.#.....#.....#.#.....#.........#.......#...#.#.#.....#.#.#.....#...#...#.#.#...........#.....#.#.#.#  ",
  "  ###.#.#####.#.###.#######.###.#.###.#####.#.#.#####.#######.###.#########.#.#.#.#####.#.###.###.#.###.#.#.###.#.#.###.#.#####.#.#.#  ",
  "  #.#...#.#.#.#.#.........#.................#.....#.#.......#...#...#.......#...#...#.#.#.........#.............#.#.#.#.#...#.......#  ",
  "  #.#.###.#.#.#######.###.#.###.###.#.###.#.#.#.###.###.#####.###.#########.###.#.###.#.#####.#######.###.###.#######.###.###.#######  ",
  "  #...#.......#.......#.....#.....#.#.#.#.#.#.#.#.....#...#.....#...#.#.........#.....#...#.#.....#.#.#.#...#...#.....#.....#.......#  ",
  "  ###.#######.###########.###.#.#######.#.#######.###.#.#######.#.###.###.#########.###.###.#.#####.###.#.###.#####.###.#.###.#.#####  ",
  "  #...........#.#.#.........#.#...#...#...#.......#.....#.......#.#...#.#.....#.....#.......#.......#.#...#.............#.....#.....#  ",
  "  #.#.#######.#.#.#.#.#####.#######.#####.#####.#.#########.#.#.#.#.#.#.#.#.#.###.#.#####.#.#.#######.###.#####.#####.#.#############  ",
  "  #.#.#.#.#.........#.#.......#...#...........#.#.#...#.....#.#.#...#...#.#.#...#.#.#.....#.............#.....#.....#.#.#.#.......#.#  ",
  "  #####.#.###.#####.#############.###.###.#######.#.#.###.#####.#######.#####.#.#.###.#.#.#.#.#####.#####.#.###.#####.#.#.#####.###.#  ",
  "  #.........#.#.#...#...#.#.#...#.#.#.#.....#.......#...#.....#.#.........#.#.#.#...#.#.#.#.#.#.#...#.#...#.#.......#.#...#.....#...#  ",
  "  #######.#####.#.#.###.#.#.#.#.#.#.#####.###.###.###.#.#.#######.#######.#.###.###.###.#######.###.#.#####.###.#.#######.#####.###.#  ",
  "  #.#.......#...#.#.#.......#.#.#...........#.#.#...#.#.#...#.......#.#.#.#.#...#...#...#.........#.#...#.....#.#.#.#.#.#.#.#...#...#  ",
  "  #.#.#.#.#####.#########.#####.#####.#.#.#####.#.#####.#.###.###.###.#.###.#.###.#####.#.###########.#####.#######.#.#.###.#.###.###  ",
  "  #...#.#.#...#.....#.#.....#...#.#...#.#...#.....#.....#.#.....#.........#.....#.....#...#.............#.#.#.#.#.#...#.#.#.....#.#.#  ",
  "  #######.###.#.#####.#.#.#.###.#.#####.###.###.#.#####.#.#######.#.#.#.#####.#.#.#.#.#.#.#.###.###.#.#.#.###.#.#.#.###.#.###.###.#.#  ",
  "  #.#...................#.#.........#...#.#...#.#...#...#.....#...#.#.#...#...#.#.#.#.#.#.#.#.....#.#.#.......#.#...#.#.#...#.......#  ",
  "  #.#.###.#####.#.#.###########.#.###.###.#.#####.#.###.#.#########.###.#######.#.#.###.###.###.###########.###.#.###.#.#.###.#.#.###  ",
  "  #...#.#.#.....#.#.#...#.#.....#.#.#.#.#.......#.#.#.#.#.....#.#...#.....#...#.#.#.#.....#...#.#.#.#.#.#...#.#.....#.#.#...#.#.#.#.#  ",
  "  #####.#.###.#.#####.#.#.#######.#.###.#.#.###.#####.#.#####.#.#######.###.###.#.#####.###.#####.#.#.#.#####.#.###.#.#.#.###.#####.#  ",
  "  #.......#.#.#.#.#.#.#.#...#.............#.#...#.....#.#.........#.......#.....#.....#...........#.#.#.......#.#.#.......#.#.......#  ",
  "  #########.#####.#.###.###.###.#.#.#####.#######.#####.#.#####.#######.#####.#.#.#######.#.###.###.#.#.#########.#######.#.###.#####  ",
  "  #.....#...#...#...#...#.#...#.#.#.#.#.....#.........#.#...#.#.#...#.#.#.#...#.#.#.......#...#.............#...#.#.........#.#.....#  ",
  "  #.#.#####.###.#.###.###.###.#.#.###.#####.###.#.###.#.###.#.#####.#.#.#.#####.#.#####.#.#.#.#.###.#.#########.#.###.#.#.###.#.#####  ",
  "  #.#...#.....#.#.........#.....#.#...........#.#.#...#.#...........#...#.......#.#.....#.#.#.#.#.#.#.....#.#...#...#.#.#.......#.#.#  ",
  "  ###.#####.#.#.#####.#.#.###.###.###.#.#.###.###.#.###.#.#####.#######.#.#####.#.#####.#.#####.#.#####.###.###.#.#####.###.#.###.#.#  ",
  "  #.......#.#.#.#...#.#.#.#.....#.#...#.#.#...#...#.#...#.#...#.#.....#.#.....#.#.....#.#.#...#...#.#.#...#.#.........#.#.#.#.....#.#  ",
  "  ###.#######.#.###.#####.###.#########.#.#######.#.#.###.#.#.###.###.#.#.###.###.#####.###.###.###.#.#####.#.#####.#####.#####.###.#  ",
  "  #.#...#.#.....#.....#.....#.#.........#.....#...#.#...#...#.#...#.#.#.#.#.....#...#.........#.#.#.#...#...#.#.#.#.#.#.#...........#  ",
  "  #.#.###.###.#.#####.###.#############.#.#.#####.#.###.###.#####.#.#.#.#######.#.#######.#.#####.#.#.#####.###.#.#.#.#.###.###.#####  ",
  "  #...#.#.#...#...#.#.#.#.......#.....#.#.#.#.....#.#...#.......#.#.#.....#.#...#.#.......#.#...#.#...#.#.#...#.#.#...#.#.#.#.......#  ",
  "  ###.#.#.#.#######.#.#.###.#######.#####.#####.#.#.#.#.#.#######.#########.#.#.#.#####.#####.###.#.###.#.#.###.#.#.###.#.###.#.#####  ",
  "  #...#.....#.#.#...#...#.#.#.#.#...........#...#.#...#.#.......#...#.........#.#.....#.........#.......#.#...#...#...#...#.#.#.....#  ",
  "  #.#.#####.#.#.###.#.###.#.#.#.#.#######.#######.#########.#.###.#######.###########.#######.#####.#.###.#.#####.#.#####.#.###.#####  ",
  "  #.#...#...#.#.#.....#.......#.#...#    E       S         Z J   C       A           R       X    #.#...#...#.......#.#.#.#.#.....#.#  ",
  "  ###.#####.#.#.###.#######.###.#####    L       Y         M G   D       U           D       A    #.#######.#####.###.#.#.#.#####.#.#  ",
  "  #.#.#.#.....#.#...#...#.....#.#...#                                                             #.......#...#.....#.#...#...#......XA",
  "  #.#.#.###.###.###.#.#####.###.###.#                                                             #.###.#.#.#.#.###.#.#.#.#.#.#.#.#.#  ",
  "  #.............#.....#.......#.....#                                                           WC..#...#.#.#.#...#.....#...#...#.#.#  ",
  "  #.#.###.###.###.###.###.###.#.###.#                                                             ###.#.###.#####.#######.###.###.#.#  ",
  "RD..#...#.#.......#.....#...#...#.#.#                                                             #...#.#...#...#.#...#...#...#...#.#  ",
  "  #.#########.###.###.#.###.#.#.#.#.#                                                             ###.###.#.#.#.#.#.#########.#####.#  ",
  "  #...#.....#...#...#.#.....#.#.#...#                                                             #.......#...#.....#.....#.....#...#  ",
  "  #.#####.#######.#.###.###.#.#.#.###                                                             #.#############.###.#############.#  ",
  "  #.#...#.#.....#.#.#.#.#.#.#.#.#....UZ                                                           #.#.........#.#.#.#.#.#.#.....#.#.#  ",
  "  #####.#.#.#######.#.###.#.#.#####.#                                                             ###.#######.#.#.#.#.#.#.#.#.###.###  ",
  "  #.....#.....#...#.#.#...#.#...#...#                                                           TR..#.#...#.....#...#...#.#.#.#.#...#  ",
  "  #.###.#.#######.#.#.###.###.#######                                                             #.#.#.#.#.#.#.###.#.###.###.#.###.#  ",
  "ZM....#.......#.#.#.#.#.....#.#.#...#                                                             #.#.#.#...#.#...#.#................EL",
  "  ###.###.#####.#.###.###.#.###.###.#                                                             #.#.#.#############.#########.#.#.#  ",
  "  #...#.#...#.......#.....#.........#                                                             #...#...............#.......#.#.#.#  ",
  "  ###.#.#.#####.###.#.###.#.#######.#                                                             #.#.#######################.#######  ",
  "  #.#.#.#.......#.......#.#.....#....UH                                                           #.#.#.........#.....#.#...#...#.#..AU",
  "  #.###.#############################                                                             #######.###.#####.###.#.###.#.#.#.#  ",
  "  #.......#.............#...........#                                                           TZ..........#.................#......ZZ",
  "  #.#####.#.#.#.#.###.#####.#.#.###.#                                                             ###################.#.#####.#.#####  ",
  "  #.....#...#.#.#.#.#...#.#.#.#...#..WT                                                         ET..#.#.......#.#...#.#.....#.#.#...#  ",
  "  #####.###.#.#######.###.#.#.#####.#                                                             #.#.#.###.###.#.###############.###  ",
  "  #.....#...#...#.....#.....#...#...#                                                             #.#...#.........#.#.#.....#.#......TZ",
  "  #.#.#.#.#.#######.###.###.#########                                                             #.#.###.#.###.###.#.#.#.#.#.#.#.###  ",
  "TR..#.#.#.#...#.........#.....#.....#                                                             #.#.#...#...#.#.#.....#.#.#...#.#.#  ",
  "  #############.###.###########.#####                                                             #.#.#.#########.#####.#.#####.###.#  ",
  "  #...#...#...#.#...#...........#.#.#                                                             #...#.................#...........#  ",
  "  #.#.#.#.###.###.#####.#.###.###.#.#                                                             #.#################################  ",
  "WT..#...#.....#...#.#...#.#...#.#...#                                                             #.#.......#...#...#...............#  ",
  "  #.###.#.###.#####.###.###.###.###.#                                                             ###.###.###.#.###.#.#.#.#.#####.#.#  ",
  "  #.#...#.#.....#...#...#.#.......#.#                                                           QS..#.#.#.....#.......#.#.#.#.....#.#  ",
  "  #####.#.###.#####.#.#.#.#######.#.#                                                             #.#.#.#####.###.#.#.#########.###.#  ",
  "  #...#.#.#...........#.#.#.#.#......WE                                                           #.#.......#.#.#.#.#.#.......#.#.#..JG",
  "  #.#######.###.#.#######.#.#.#######                                                             #.#######.###.#####.#####.#####.#.#  ",
  "  #.....#...#...#.#.#.......#.......#                                                             #.............#.#.#.#.#...#...#...#  ",
  "  #.#.#.#######.#.#.###.###.###.###.#                                                             ###.###########.#.###.###.###.#####  ",
  "UZ..#.#.#.#...#.#.#.#...#...#...#...#                                                           XR..#.#...........#.#.#.#...........#  ",
  "  ###.###.#.#.#.###.###.#.###.#.###.#                                                             #.###.###.#####.#.#.#.###.#.###.###  ",
  "  #.....#...#.#.#.....#.#...#.#.#.#..AQ                                                           #.#...#.....#.............#.#......WC",
  "  ###.###.#.#####.###.#.###.###.#.###                                                             #.###.###.#######.#.#.#.###.#.###.#  ",
  "  #.#.....#.........#...#.........#.#                                                             #.#.#.#.#.#.......#.#.#...#.#.#.#.#  ",
  "  #.#########.###.#####.###.###.###.#                                                             #.#.#.#.#######################.###  ",
  "  #.......#.....#.....#...#.#.#...#..LS                                                           #.......#.#.......#.....#...#.#.#.#  ",
  "  #.###.#################.#.#.#####.#                                                             #######.#.#.#######.#####.###.#.#.#  ",
  "  #.#.......#.#...#.#.#...#.#.......#                                                             #.#.#.#.#...#.........#.........#.#  ",
  "  #.#.#######.###.#.#.#######.###.#.#                                                             #.#.#.###.#.#.#######.#.#.#.###.#.#  ",
  "SY..#.....#.#.....#...#.#...#.#.#.#.#                                                           ER..........#.#.#.#.#.....#.#.#...#..XR",
  "  ###.#.###.#.###.###.#.#.#####.###.#                                                             #.#####.###.#.#.#.#.###.#######.#.#  ",
  "  #...#.......#.....................#                                                             #.#.#...#...#.....#.#.#.#.#.....#.#  ",
  "  #######.###.#######.#.#.###.#.#.#.#                                                             #.#.#######.###.#.###.#.#.#####.#.#  ",
  "  #.........#.....#...#.#.#.#.#.#.#.#                                                             #.....#.#.......#.#...........#...#  ",
  "  #.#####.#.#######.#.#.#.#.#####.#.#        L         E         I       K       N     A          #.#####.###.#.###.#.#####.#######.#  ",
  "  #...#.#.#...#.#...#.#.#.....#.#.#.#        Z         M         F       F       T     V          #...#.......#...#.#...#.......#...#  ",
  "  #####.#.###.#.###.#.#.#.#.#.#.#############.#########.#########.#######.#######.#####.#################.#.###.#.#####.#.###.#####.#  ",
  "  #.......#.......#.#.#.#.#.#.....#...#...........#.#...#.#.......#.#.#.........#...#.......#...#.#.....#.#.#.#.#.....#.#.#.....#...#  ",
  "  #.###.#.#.#.#####.###.###.#.#.#.###.#.#####.#.###.#.#.#.###.#####.#.#####.#.#####.#######.#.###.###.#######.#.#.#######.#####.#.#.#  ",
  "  #.#...#.#.#.....#.#.....#.#.#.#...#...#.#...#.....#.#.#.....#.....#.......#.#.#.#.#.....#...#.....#.#.#.......#.#.#.........#.#.#.#  ",
  "  #####.#.###.###.#.###.###.#.#.#########.#.#.#.###.#.#######.#.#.#.#.#.#.#.###.#.#.###.#.#.#####.###.#.#####.###.#.#####.#####.###.#  ",
  "  #...#.#.#.....#.#.#.....#.#.#.#.#.....#...#.#.#...#...#...#.#.#.#.#.#.#.#.#.#.#...#...#.................#...#.........#.#.......#.#  ",
  "  #.#.#.#.###.#####.#.#.#.###.#.#.#.#.#########.###.#.#####.#.#.#.#######.###.#.#.###.#####.#####.#.#.###.###.#.#####.#######.#.###.#  ",
  "  #.#...#.#.....#...#.#.#...#.#...#.#.....#.....#...#.#.#.......#...#.#.#...#.......#.#.#.#...#.#.#.#.#.....#.#.....#...#.....#...#.#  ",
  "  #.###.#.#.#.#.###.###.#######.#######.#.#.###.###.#.#.###########.#.#.#.###.#######.#.#.#####.###########.#####.###.#########.###.#  ",
  "  #.#...#.#.#.#...#.#.....#.....#.#.....#.#...#.#...#.#.....#.......#.......#...#.................#...#.....#.......#.#.#.#.#.....#.#  ",
  "  #####.###.#.###.#####.#.###.#.#.#######.#########.#.#.###.###.#####.#######.#.###.#.###.###.###.#.###########.#.#.###.#.#.#.#####.#  ",
  "  #.....#.#.#.#.#.#.#...#.#.#.#.#.#.#.........#.#.#.#.#.#.#.#.......#.#.....#.#.#...#.#...#.....#.#...#...#.#...#.#.........#...#...#  ",
  "  #.###.#.#####.#.#.#.#.#.#.#####.#.#####.###.#.#.#.#.#.#.#.#####.#.#.#.#.###.###.###.###.#########.#.#.###.###.###.#####.###.###.###  ",
  "  #.#.......#.......#.#.#.#...#...........#.........#.....#.#.#.#.#.#.#.#.#.#...#...#.#...........#.#.#.......#.#.#.....#...#.#...#.#  ",
  "  #####.#.#####.###.###.###.#####.#.#.#.#.#######.#########.#.#.###.#.#.#.#.#.#.###.#.#####.#####.#.#####.#.#####.#.#####.#.#####.#.#  ",
  "  #.....#.#...#...#.#...#.....#...#.#.#.#...#.#.....#.............#.#...#...#.#.#.#.#.....#.....#.........#...#.........#.#.....#...#  ",
  "  #.#.#.#.###.#####.#########.#####.#########.#####.#.###.#####.###.###.#.###.###.#########.###.#.#######.###.###.###.#.#.#.###.#.#.#  ",
  "  #.#.#.#.#...............#.......#...#...#...#.#...#.#.#...#...#.....#.#...#.#.........#.....#.#.....#...#.....#...#.#.#.#.#.#.#.#.#  ",
  "  #.#.###.#.#.###.#.###.#.#####.#######.#####.#.###.#.#.###.#######.#######.#.#####.#.###.#.#####.###.#######.#.#.#.#.#.#####.###.###  ",
  "  #.#.#.#.#.#...#.#.#...#.#.........#.....#...#.....#.....#.#...#...#.#.#...#.#.#...#.#.#.#.....#...#.#...#...#.#.#.#.#.........#...#  ",
  "  #.###.#.###.#.###.###.###.###.###.#####.###.#.#######.#######.#.###.#.###.#.#.###.###.###.#####.#######.###.###.###.###.#.#.###.#.#  ",
  "  #.#.......#.#.#.....#...#.#.#.#...#.#...#.........#...#.#.#...#.....#.#...#...........#.......#.#.........#.#.#.#.....#.#.#.#...#.#  ",
  "  #.###.###.#.###.#.#########.#####.#.#.#####.###.###.###.#.###.###.###.###.#.#####.#.#.#.###.###.###.#.###.###.#######.#.###.###.###  ",
  "  #.#...#...#.#...#...#.#...#.................#.#.#.......#.......#...#.....#.#.#.#.#.#.#.#.#.#.......#.#.#...#.#...#...#.#...#.....#  ",
  "  #.#.#.#.#.###.###.###.###.#######.###.#.#.#.#.#.#####.#####.###.#.#######.#.#.#.#.#####.#.#.###.#.#####.#####.#.#######.#.#####.###  ",
  "  #.#.#.#.#.#.....#...#...#.#.......#...#.#.#...#.#.#...#.#...#.#...#...#...#.#.........#...#...#.#...#...#.#.....#...#...#...#.....#  ",
  "  #####.#######.###.###.###.#.#.#######.#######.###.#.#.#.###.#######.#.#.#####.#.#.#######.#######.#####.#.###.#####.#############.#  ",
  "  #.....#.......#.#.#.........#.#.#.....#...#.......#.#.#.#.#.#...#...#.#.....#.#.#.#.#...#.#.#.#...#...#.#.........#.....#.#...#.#.#  ",
  "  ###.#######.#.#.#####.#########.###.#.###.###.###.#.###.#.#.#.#.#.#.#.###.#####.###.###.#.#.#.#######.#.#.#.#.#.###.#####.#.###.###  ",
  "  #.....#...#.#.#...........#.........#.#.......#...#.#.....#...#.#.#.#...#.#.......#.#.#.....#.#...#.......#.#.#.#.#...............#  ",
  "  #.#####.#######.#.###.#.#####.###.###.#####.###.###.#.#####.#######.###.#.#.#######.#.#.###.#.###.#.#############.#.#.###.#######.#  ",
  "  #...#...........#...#.#...#...#.#.#.#.#.#.#.#.....#...#.#.#.....#.....#...#.....#.#.......#.....#...#...#...#...#...#...#...#.....#  ",
  "  #.#.#.#####.#####.###.#.###.###.###.#.#.#.#####.#####.#.#.#.#########.#####.#.#.#.#.#.#.#####.###.###.###.###.#####.###.###.###.###  ",
  "  #.#.#...#.....#...#...#.#.....#.......#.........#.......#.......#.........#.#.#...#.#.#.....#.......................#.....#.#.....#  ",
  "  ###############################################.#######.#####.#########.#######.#.#.###############################################  ",
  "                                                 A       L     N         E       A E E                                                 ",
  "                                                 Q       S     T         R       A M T                                                 ",
];

fn find_portals(map: &[Vec<char>]) -> Vec<(String, usize, usize)> {
  let mut portals = Vec::new();
  for i in 1..(map.len()-1) {
    for j in 1..(map[0].len() - 1) {
      let from = map[i][j];
      if !from.is_ascii_uppercase() { continue; }
      let neighbours = [
        (map[i-1][j], i-1, j),
        (map[i+1][j], i+1, j),
        (map[i][j-1], i, j-1),
        (map[i][j+1], i, j+1),
      ];
      if let Some(&(_,x,y)) = neighbours.iter().find(|(c,_,_)| *c == '.') {
        let to = neighbours.iter().find(|(c,_,_)| c.is_ascii_uppercase()).unwrap().0;
        let name = [from, to].iter().sorted().collect();
        portals.push((name, x, y));
      }
    }
  }
  portals
}

fn path_len(
  g: &HashMap<(usize,usize), Vec<(usize,usize)>>,
  (x, y): (usize,usize),
  end: (usize,usize),
) -> usize {
  let mut q = g.iter().map(|(pos,_)| *pos).collect::<HashSet<_>>();
  let mut dist = q.iter().map(|&pos| (pos, 9999)).collect::<HashMap<_,_>>();
  dist.insert((x,y), 0);

  loop {
    match q.iter().min_by_key(|pos| dist[pos]) {
      Some(&u) => {
        q.remove(&u);
        if u == end { break; }

        for v in &g[&u] {
          if !q.contains(&v) { continue; }
          let d = dist.get(&u).unwrap() + 1;
          if d < *dist.get(&v).unwrap() { dist.insert(*v, d); }
        }
      }
      None => unreachable!(),
    }
  }
  dist[&end]
}

fn main() {
  let now = Instant::now();
  let map = INPUT.iter().map(|s| s.chars().collect_vec()).collect_vec();

  let mut g = HashMap::new();
  for i in 0..map.len() {
    for j in 0..map[0].len() {
      if map[i][j] != '.' { continue; }
      let mut neighbours = Vec::new();
      if map[i-1][j] == '.' { neighbours.push((i-1, j)); }
      if map[i+1][j] == '.' { neighbours.push((i+1, j)); }
      if map[i][j-1] == '.' { neighbours.push((i, j-1)); }
      if map[i][j+1] == '.' { neighbours.push((i, j+1)); }
      g.insert((i,j), neighbours);
    }
  }

  let portals = find_portals(&map);
  for index in 0..portals.len() {
    let (p1, x, y) = &portals[index];
    if p1 == "AA" || p1 == "ZZ" { continue; }
    let (_,i,j) = *portals.iter().find(|(p2,x2,y2)| p1 == p2 && x != x2 && y != y2).unwrap();
    g.get_mut(&(*x,*y)).unwrap().push((i,j));
  }

  let (_,x1,y1) = *portals.iter().find(|(p,_,_)| p == "AA").unwrap();
  let (_,x2,y2) = *portals.iter().find(|(p,_,_)| p == "ZZ").unwrap();

  let answer = path_len(&g, (x1,y1), (x2,y2));

  println!("Answer: {}", answer);
  println!("Time: {}ms", now.elapsed().as_millis());
}