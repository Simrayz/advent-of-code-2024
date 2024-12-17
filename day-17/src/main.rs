use day_17::*;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    let input: &str = include_str!("../input-puzzle.txt");

    let solution = part1::process(input)?;
    println!("Part 1: {}", solution);

    let solution = part2::process(input, true)?;
    println!("Part 2: {}", solution);

    Ok(())
}
