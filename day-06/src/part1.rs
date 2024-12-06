use crate::board::Board;

pub fn process(input: &str) -> miette::Result<String> {
    let board = Board::new(input);

    let result = board.find_unique_positions().len();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input1.txt"));
        assert_eq!("41", result?);
        Ok(())
    }
}
