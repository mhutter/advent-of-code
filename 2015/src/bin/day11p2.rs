use common::generate;

use advent_of_code_2015::day11::day11p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day11.txt"));
    dbg!(day11p2(&i));
}
