use common::generate;

use advent_of_code_2015::day03::day03p2;

fn main() {
    let i = generate::chars(include_str!("../../input/day03.txt"));
    dbg!(day03p2(&i));
}
