pub fn process(_input: &str) -> miette::Result<String> {
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input-example.txt"));
        assert_eq!("", result?);
        Ok(())
    }
}
