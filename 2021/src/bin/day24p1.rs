use common::generate;

use advent_of_code_2021::day24::day24p1;

fn main() {
    let i = generate::ints(include_str!("../../input/day24.txt"));
    dbg!(day24p1(&i));
}
