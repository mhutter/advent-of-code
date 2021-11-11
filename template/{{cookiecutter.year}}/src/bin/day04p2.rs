use common::generate;

use advent_of_code_{{cookiecutter.year}}::day04::day04p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day04.txt"));
    dbg!(day04p2(&i));
}
