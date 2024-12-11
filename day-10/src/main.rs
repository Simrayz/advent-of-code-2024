use day_10::*;

fn main() -> miette::Result<()> {
    let input: &str = include_str!("../input1.txt");

    let solution = part1::process(input)?;
    println!("Part 1: {}", solution);

    let solution = part2::process(input)?;
    println!("Part 2: {}", solution);

    Ok(())
}
