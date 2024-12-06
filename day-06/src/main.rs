use day_06::*;

fn main() -> miette::Result<()> {
    let input: &str = include_str!("../input2.txt");
    let solution = part1::process(input)?;

    println!("Part 1: {}", solution);

    Ok(())
}
