use common::generate;

use advent_of_code_2015::day04::day04p1;

fn main() {
    let i = generate::ints(include_str!("../../input/day04.txt"));
    dbg!(day04p1(&i));
}
