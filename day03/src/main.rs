// input is in parse() 
use std::time::{Instant};

fn is_obstacle(map: &String, width: usize, x: usize, y: usize) -> bool {
    let y_translated = y - 1;
    let x_translated = (x - 1) % (width);
    let slice_start = y_translated * (width + 1); // must add one because of newline
    let slice_end = slice_start + width; // excludes the newline already
    let row = &map[slice_start..slice_end];
    // println!("row {} is {:?}", y, row);
    let point = &row[x_translated..x_translated + 1];
    // println!("point for x {} is {}", x, point);
    point.contains('#')
}

fn part_a(map: &String, width: usize, height: usize) {
    let mut count = 0;
    // coords start with 1, we don't care about first row, but we care about last row
    // coordinate start at (1,1) next checked position is (4,2), another row (7,3)
    // the slope right 3, down 1

    for y in 2..height + 1 { 
        let x = (y-1) * 3 + 1;
        
        if is_obstacle(map, width, x, y) {
            count += 1;
        }
    }
    println!("answer a: {}", count);
}

fn count_for_slope(map: &String, width: usize, height: usize, step_right: usize, step_down: usize) -> u64 {
    let mut count = 0;
    let mut y = 1 + step_down;
    loop {
        let x = 1 + (y - 1) * step_right / step_down;
        // let x = if step_down == 1 { (y - 1) * step_right / step_down + 1 } else { };
        // println!("x is {} y is {}", x ,y );
        if is_obstacle(map, width, x, y) {
            count += 1;
        }
        y += step_down; 
        if y >= height + 1 {
            break;
        }
    }
    count
}

fn part_b(map: &String, width: usize, height: usize) {
    let mut numbers: Vec<u64> = Vec::new();
    numbers.push(count_for_slope(map, width, height, 1, 1));
    numbers.push(count_for_slope(map, width, height, 3, 1)); // identical to part_A
    numbers.push(count_for_slope(map, width, height, 5, 1));
    numbers.push(count_for_slope(map, width, height, 7, 1));
    numbers.push(count_for_slope(map, width, height, 1, 2));

    let mut answer = 1;
    for num in numbers {
        answer *= num;
    }
    println!("answer b: {}", answer);
}

fn main() {
    let map = parse();
    // let map = _example();
    let width = map.find("\n").unwrap();
    let height = map.len() / (width + 1); // newline character
    println!("width {} height {}", width, height);
    
    let start_a = Instant::now();
    part_a(&map, width, height);
    let start_b = Instant::now();
    part_b(&map, width, height);
    println!(" A took {:?}", start_a.elapsed());
    println!(" B took {:?}", start_b.elapsed());
}

fn _example() -> String {
    let input = r########"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"########;
input.to_string()
}

fn parse() -> String {
    let input = r########".....#.##......#..##..........#
##.#.##..#............##....#..
3.....###...#..............#.##
4....#..##.#..#......#.#.#..#.E
5.#.......###..#..........#.#..
..#..#.##.......##.....#....#..
.##....##....##.###.....###..#.
..##....#...##..#....#.#.#.....
.....##..###.##...............#
#.....#..#....#.##...####..#...
#......#.#....#..#.##....#..#.#
##.#...#.#............#......#.
.#####.......#..#.#....#......#
..#.#....#.#.##...#.##...##....
.....#.#...#..####.##..#.......
#....#...##.#.#.##.#..##.....#.
##.##...#....#...#......#..##..
....##...#..#.#...#.#.#.....##.
..#....##......##....#.#....#..
#..#....#....###..#.##....#.#.#
..#.#####..##....#....#.....##.
.#...##.......#...#....#.#...##
#.#.#.##.......#.....#.#.#....#
.#.#.....#.......#.......##....
.#......#....#....#.......##...
#......#.....#......#..#..#....
#.#...#...#....##....#.#...#..#
....#.....##...#...#..#.#......
..#......#..........#...#.#....
..#..#......####..##...###.....
.#.....#...##...#.##........###
#.#....#..#....#..#.....#.#..#.
...##.##.#.#.##...#.....#......
##....#.#.#...####.#.#.#.#.....
.##.........#..#..###..........
..##.###.#..#..#....##.....#...
##........#..###....#.#..#..#..
....#.#.......##..#.#.#.#......
....##.....#.........##.......#
..#........##.#.........###..##
....#..................##..#...
#...#.#..###..#.....#..#..#...#
..#..#.##..#..#.......#.......#
.....#..##..#....##...........#
..##...#........#...#.#.......#
.........#.#..#.#..#.##.#.###..
....#...#..#..#......##....#.#.
..#..#.#....#....#..#.####..##.
##....#.....#......##.###.#..#.
#..#..##..###......#.#.#.#...#.
.......#..##..##...#...#..#....
..#.###.#...#....#.##.#.....##.
.#.#.......##...##...##....#...
#...#.#.#...#.####..#..##......
###..#.##..#..........#...#....
##.#.........#..##......####...
..##.#..#....#.##..............
...#....#.......###............
...#.....##....#.#.#.#.......##
###.###...#...#...###.##...##..
#.#....#.##..#.....#.....##.#..
...#....#....#.........#....#.#
##.#....#........#..#..##.#....
.#.#..#.......#...##.......#...
.##...##........#....#.#..#....
....#..#.##.###.....#.#........
.#.#...#.#..#.....#.........#..
.......#.#.#..##....#.........#
.##...#....#..#...#........#..#
....#....#..#.#..#.#.#....##.##
..##....#.....##..#.#...#...#..
#.##.........#.....#.......#.##
...#...##.#.#..........#......#
###...#.....#..#.......#####..#
#.####...##.#.#..#...#.........
.##.....#.....##..#...##.##....
.........###...#......##....###
.#....##...###.#..#...##..#.#.#
.......#.......#.#...##.#......
.....#.#........#..##.....##...
....#.#.........##.#...##..#.#.
#..#..#.##..#.##.##.....##.###.
..##.........###...#....#....#.
.###...#..#.##...........#.....
#..##..........#..........#....
.....#.#....#..##..#...#.#....#
..#.....#.#....#...##.##.......
##.....##........#....#..##....
.#..#.#.........#..#..#........
.............##....#....#..#...
....##....#..#.#.##....###.##.#
.###..#.....#..#..##..#..##..#.
...#..###.......#.#....#..###..
#.#..#.....#...#......#........
#..#..............###.#......#.
..#....##.#....#.##.#.#...#....
.........##..#...#.#.......#...
........#...#.#....#.....##..#.
...#.##..#..#..###..#..#......#
.....####......#...#....#...#.#
...###.#.#......#....#.......#.
#...##.#....#....##....##.###..
.......##...##.....#.##.#..#..#
.....#.#............##...#.####
.##..#.#.#.#..#.#.#.....#.##...
.#..####...#.#....#.....#..#...
....##..#.#...#..#....#.#......
...#......###..#..###..#.....#.
.#.#.#..##....#...##..#.....#..
###....#....#...##.....#...#...
#.##....#......#...###.........
.#..#.#...#..#....#....#....#..
...............##...####..#..#.
#.#...........####..#...##.....
##.#....#........#......#...##.
......#...#...#....#....#.....#
#......#.............#....###..
.#...#...##.....#...##.##..#...
..#.#......#.#........#........
.......#..#.#...##..#.#.#......
..##...#.##........#....#.#...#
.....#..#..#........#.#......##
....#.#...##............##....#
.#.#....#.#.#...#...#.##.....#.
#.#.##...#....#.#.#..#.##..#.#.
.........####..#...#...#.......
#..#..####......#..##..#...#...
.........##..................#.
.....##.#..##.#.#...#......##..
...#....#....#.#.....#...#..#.#
#...##.#...##...........#..#...
#..........#.#..#..#.##..#..#.#
.#...#.##...#.#.#..#.......##..
.........#...........#..#..#...
.##...##....#.#......#........#
#.#...........#....#.......#...
##.#.#.......#...###......##..#
...###..#.##..##.#.#.......#...
.#...#..##.#...#........#.....#
...#.......#..#..........#.#...
..#.#.#.#.....#.#.......#..#..#
#.##.....#..##...#..###.#....#.
.......#...........#...#....###
.......#..#...#.............#..
#.....###.......#...#........#.
.#..#..#..#...........#........
....#.#...#.#.##.#.#....#.##..#
.......#..##...##...#...#......
...#.....##.###...#.#...##....#
#..#....#...##......#....##....
#.#.......#....#.###.##..#..#..
..##...........#...#....#......
.#........#.....#..#..#...#..##
.....#.#.#..#.......#....#.....
#..#.#......#......##....#.....
##.....................##......
.##........###..#.........#...#
........#.........#..#.........
.#.##....#.....#...#.........##
....##......#.........#........
...#.#..#...##.##.#.#..####....
..##...........##.#.#....#.....
.#.....#.#...#..#.......#....#.
....#...#......##...#...##.#..#
....#..##....#..#.........##.#.
..##...##.##....#....##.###...#
..#....##..##.#.#.#...#......#.
##...#.........#...........#...
.##....##.#.....#...#.......#..
..........##.###.##....###....#
..........#..##..#....#.#.##.##
........##.#...#.#.#.#...###.#.
.#......#.#.#...###.#.#.#......
.........#......#......#...#..#
......#.....#.##....##.#####..#
..#..##...###.#..........#.#.#.
.#..#....###.#...#..#....#...##
...................#..........#
....###.....#...##......#.....#
#.....#..##.....#.#..........#.
..#.......##.#....#..#.##.#...#
........##.#..###..#......##...
#...........##.#...###..#....#.
....#...........#.....#.#...#..
.##..#.#...#...#.##...#..#.....
#........#.#.#.#.#.#...........
#..#.....#..#..#.##....#....#.#
..#............##....#.#.##...#
.....###.#....#.#......#.###...
...#.....#.#.................#.
..#...##..#.#...#...#...#.....#
.##.#........#..#....##..#..##.
.#..........#...#.#..#..#.#....
#.......##.........#.##..#.####
.#..............#.......##.....
#......#.##..........#..#......
..##...#...#.#...#............#
.##.##..##..##........##.....#.
.....#..#.....##...............
.#..#...##...#...#.....#.......
#......#...#.......#..##.###.##
###..##......##......###....#..
....#..........#...#.##.#.....#
.........#....#..#..#.#..##....
.....#.....#...........#......#
.#.......#...#....##...#.##...#
..##.#..............#..#...#.#.
.#..####.#.........#....#....#.
..###.#...#..#......#.......###
.#.#..##...###...#...#.#...#.#.
...#..##..###.#..#.....#.##....
#...###.#...##.....####.....#..
.#.##...#..#.#..##.....#.......
...#.##.....##.....#....#......
.#...##.....#..###..#..........
..........#...#.....#....##.#..
.......#...#...#...#........#..
#....##..#...#..##.#.#.....#...
.#.#..............#..#....#....
.####.#.#.###......#...#.#....#
.#...#...##.#...............#.#
...#.......##...#...#....##....
#..........###.##..........##.#
.......#...#....#.#..#.#....#..
....#.##.#...###..#..##.##.....
..#.#.#......#.#.......###.....
#..................#.##....#...
#.....#..#.#.#..#...#.........#
..#..#...#.#.##........#.......
#..#.#..#..........###...#.#...
.......#.##....#........##.#...
.####.#.#...#.#...##.##.....###
........#.#...#.#..##...##.....
....##.##......#.##.........#..
.#..#...#.#...........#........
.......#..........#....#...#...
..###.#.###..#..#.....#..##....
.#..........#.......##...#.....
.#.....#...#........#...#.##..#
.#..#.......#..#.......#.#.#...
....#..##.#...##...#.#....#....
.....#.........#..#..#....#....
..#.#..##....#..#..##.#.#.....#
........#.#...###....#.#.#.....
.#.....#.......#..###.#........
.......#...#.#...#...##........
##.............#.#.....#.#..#..
.#....#.......#.#.......#..##..
#.....#........#..##..##.......
...........#.........###......#
....#.##...#.#...#...#....#..##
......#..##......#......#.##.#.
......##....####...###...#.....
#....#..........#.#.##.....#..#
....#.#...........#.#.#.#.#...#
....####.....##...#..##..#...#.
#....#.###..###.....#..###.....
..##.........#......#...##.#...
..#.....#.#...#.##.#...#..###.#
..#.##..##........#.......#.###
.....#..........#.....#....#...
.......##..##..###.......#####.
..###......#.#....###....##...#
#..##.....#..###...#.....##.##.
#..#..##.##.###.####.##.#......
.#.#......#.##......#..#......#
..###.....#.#......#.#.####....
#..............#..#.#...#.###..
...#..#.##..........##.#...#.##
.#.#.#.........#....#.#..#.....
..#.##..#...#..#...#......#....
.......#...#.##.#.#..#...##..#.
..........#.####...#........#.#
....#...#....##.#.........#.#..
##.#..#.......###....#..#..#.#.
..##.....#..#.#.#.####......#..
.#.....#..........#..#..#.#....
......#.#.......#.#...#..#..#..
...#...............#....#...#..
##.......#.........#.......#...
...#.......###...#.#...#.......
#...###....#....##....#....#...
...#....##..#.#.............##.
.....#.#.#..#......#...#.#..#..
.##....#..##..#####..##.....##.
....##.#.#..#.....#.#...#......
...#.....##.#.#..##..#.#.......
.......#..#..#..........#......
.......#...#..#.........#.##...
..#..#..#...##..#.#....#......#
..#....#...#.#......#........#.
.#...#..#...#.#..........#.....
#..#...####..#......##.##.#.#..
.#...#.#...#.#.....#..##.#.....
..#.##.#......#.........##...#.
###..............#.............
...#...###....#..#.............
.##....#......#..#.....#..#..#.
.#..........#.....##...#..#....
....##..#.#......###.##......#.
.#..##.#.##.#...##.#......###.#
#..###.#...###..#........#.#...
#..#.#.#..#...###.##.##..#..#..
#.#..#....#.........##......#..
....###.....###....#...........
....#..##.##....##..#.....#....
.#.....#....####...#..##.#..###
.........##..#......#.#...##...
.##.......#.....#.###.#..#.#..#
.....#.#...###.....#......####.
##.#...#......#.#.#..#.####...#
.#.##.....#..#..#.............#
.#..###..#..#......#..##......#
.......#.#........##.....#.#...
#....#...#..###..#.#.....#.##..
.##.....#.#....###..#.....##...
...##....#....#...#....#.#.#...
#####..#...........###....#...#
.#.......##.##.....#....#......
.#..#.#...#..#......#...#..#.#.
....#.....##...#####..#...#...#
###.##...#.#............#....#.
.....#...#........##.........#.
"########;
    input.to_string()
}
