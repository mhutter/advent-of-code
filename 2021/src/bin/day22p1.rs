use common::generate;

use advent_of_code_2021::day22::day22p1;

fn main() {
    let i = generate::ints(include_str!("../../input/day22.txt"));
    dbg!(day22p1(&i));
}
