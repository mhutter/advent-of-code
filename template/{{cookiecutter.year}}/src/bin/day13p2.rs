use common::generate;

use advent_of_code_{{cookiecutter.year}}::day13::day13p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day13.txt"));
    dbg!(day13p2(&i));
}
