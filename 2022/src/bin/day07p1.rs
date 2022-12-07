use advent_of_code_2022::day07::day07p1;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("../../input/day07.txt");
    dbg!(day07p1(input)?);
    Ok(())
}
