use day_20::*;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    let input: &str = include_str!("../input-puzzle.txt");

    let solution = part1::process(input, 100)?;
    println!("Part 1: {}", solution);

    let solution = part2::process(input, 100)?;
    println!("Part 2: {}", solution);

    Ok(())
}
