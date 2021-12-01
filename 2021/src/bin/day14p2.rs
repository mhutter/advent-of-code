use common::generate;

use advent_of_code_2021::day14::day14p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day14.txt"));
    dbg!(day14p2(&i));
}
