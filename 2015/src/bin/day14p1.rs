use common::generate;

use advent_of_code_2015::day14::day14p1;

fn main() {
    let i = generate::ints(include_str!("../../input/day14.txt"));
    dbg!(day14p1(&i));
}
