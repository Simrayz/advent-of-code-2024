use day_18::*;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    let input: &str = include_str!("../input-puzzle.txt");

    let solution = part1::process(input, (70, 70), 1024)?;
    println!("Part 1: {}", solution);

    let solution = part2::process(input, (70, 70), 1024)?;
    println!("Part 2: {}", solution);

    Ok(())
}
