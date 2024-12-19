pub mod part1;
pub mod part2;

pub fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (colors, designs) = input.split_once("\n\n").unwrap();

    let colors = colors.split(", ").collect();
    let designs = designs.lines().collect();

    (colors, designs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_parse() -> miette::Result<()> {
        let result = parse_input(include_str!("../input-example.txt"));
        let colors = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
        let designs = vec![
            "brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb",
        ];
        assert_eq!(colors, result.0);
        assert_eq!(designs, result.1);

        assert_eq!((colors, designs), result);
        Ok(())
    }
}
