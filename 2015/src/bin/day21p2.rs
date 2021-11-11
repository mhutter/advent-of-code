use common::generate;

use advent_of_code_2015::day21::day21p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day21.txt"));
    dbg!(day21p2(&i));
}
