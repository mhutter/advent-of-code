use common::generate;

use advent_of_code_{{cookiecutter.year}}::day24::day24p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day24.txt"));
    dbg!(day24p2(&i));
}
