use common::generate;

use advent_of_code_{{cookiecutter.year}}::day11::day11p2;

fn main() {
    let i = generate::ints(include_str!("../../input/day11.txt"));
    dbg!(day11p2(&i));
}
