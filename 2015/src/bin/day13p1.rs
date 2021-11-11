use common::generate;

use advent_of_code_2015::day13::day13p1;

fn main() {
    let i = generate::ints(include_str!("../../input/day13.txt"));
    dbg!(day13p1(&i));
}
