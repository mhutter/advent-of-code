use common::generate;

use advent_of_code_2015::day15::day15p1;

fn main() {
    let i = generate::ints(include_str!("../../input/day15.txt"));
    dbg!(day15p1(&i));
}
