use common::generate;

use advent_of_code_2015::day22::day22p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day22.txt"));
    dbg!(day22p2(&i));
}
