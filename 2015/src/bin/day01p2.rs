use common::generate;

use advent_of_code_2015::day01::day01p2;

fn main() {
    let i = generate::chars(include_str!("../../input/day01.txt"));
    dbg!(day01p2(&i));
}
