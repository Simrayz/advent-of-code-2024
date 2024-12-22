use crate::{calculate_n_secrets, parse_input};

pub fn process(input: &str) -> miette::Result<String> {
    let start_numbers = parse_input::<u64>(input).unwrap();

    let result = calculate_n_secrets(start_numbers[0], 2000);

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input-example.txt"));
        assert_eq!("37327623", result?);
        Ok(())
    }
}
