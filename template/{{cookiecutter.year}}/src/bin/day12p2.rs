use common::generate;

use advent_of_code_{{cookiecutter.year}}::day12::day12p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day12.txt"));
    dbg!(day12p2(&i));
}
