use common::generate;

use advent_of_code_2015::day05::day05p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day05.txt"));
    dbg!(day05p2(&i));
}
