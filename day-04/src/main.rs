use day_04::*;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    const INPUT: &str = include_str!("../input1.txt");
    println!("XMAS count: {:?}", part1::process(INPUT)?);
    println!("Cross-MAS count: {:?}", part2::process(INPUT)?);

    Ok(())
}
