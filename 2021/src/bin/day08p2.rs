use common::generate;

use advent_of_code_2021::day08::day08p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day08.txt"));
    dbg!(day08p2(&i));
}
