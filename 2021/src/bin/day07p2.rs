use advent_of_code_2021::day07::day07p2;
use common::generate;

fn main() {
    let i = generate::int_list(include_str!("../../input/day07.txt"));
    dbg!(day07p2(&i));
}
