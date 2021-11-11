use common::generate;

use advent_of_code_{{cookiecutter.year}}::day21::day21p1;

fn main() {
    let i = generate::ints(include_str!("../../input/day21.txt"));
    dbg!(day21p1(&i));
}
