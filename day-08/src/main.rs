use day_08::*;

fn main() -> miette::Result<()> {
    let input: &str = include_str!("../input2.txt");
    let debug = false;
    let solution = part1::process(input, debug)?;

    println!("Part 1: {}", solution);

    let solution = part2::process(input, debug)?;
    println!("Part 2: {}", solution);

    Ok(())
}
