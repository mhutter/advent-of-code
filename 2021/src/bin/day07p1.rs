use advent_of_code_2021::day07::day07p1;
use common::generate;

fn main() {
    let i = generate::int_list(include_str!("../../input/day07.txt"));
    dbg!(day07p1(&i));
}
