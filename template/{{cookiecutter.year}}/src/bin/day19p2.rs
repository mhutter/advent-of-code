use common::generate;

use advent_of_code_{{cookiecutter.year}}::day19::day19p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day19.txt"));
    dbg!(day19p2(&i));
}
