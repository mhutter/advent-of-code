use common::generate;

use advent_of_code_2021::day12::day12p1;

fn main() {
    let i = generate::ints(include_str!("../../input/day12.txt"));
    dbg!(day12p1(&i));
}
