use common::generate;

use advent_of_code_2015::day08::day08p1;

fn main() {
    let i = generate::ints(include_str!("../../input/day08.txt"));
    dbg!(day08p1(&i));
}
