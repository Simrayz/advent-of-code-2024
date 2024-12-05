use day_05::{part1, part2};

fn main() -> miette::Result<()> {
    let file = include_str!("../input2.txt");
    println!("Part 1: {}", part1::process(file)?);
    println!("Part 2: {}", part2::process(file)?);

    Ok(())
}
