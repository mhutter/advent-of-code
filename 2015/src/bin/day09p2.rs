use common::generate;

use advent_of_code_2015::day09::day09p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day09.txt"));
    dbg!(day09p2(&i));
}
