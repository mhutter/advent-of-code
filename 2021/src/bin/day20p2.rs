use common::generate;

use advent_of_code_2021::day20::day20p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day20.txt"));
    dbg!(day20p2(&i));
}
