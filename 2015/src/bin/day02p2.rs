use common::generate;

use advent_of_code_2015::day02::day02p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day02.txt"));
    dbg!(day02p2(&i));
}
