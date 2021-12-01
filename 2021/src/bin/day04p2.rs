use common::generate;

use advent_of_code_2021::day04::day04p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day04.txt"));
    dbg!(day04p2(&i));
}
