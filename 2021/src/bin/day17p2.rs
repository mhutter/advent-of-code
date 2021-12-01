use common::generate;

use advent_of_code_2021::day17::day17p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day17.txt"));
    dbg!(day17p2(&i));
}
