pub mod part1;
pub mod part2;

pub const POWERS: [u64; 3] = [64, 32, 2024];

pub fn parse_input<T: std::str::FromStr>(input: &str) -> Result<Vec<T>, T::Err> {
    input.lines().map(|x| x.parse()).collect()
}

pub fn calculate_n_secrets(secret: u64, time: usize) -> u64 {
    (0..time).fold(secret, |acc, _| calculate_next(acc))
}

pub fn calculate_next(mut secret: u64) -> u64 {
    secret = ((secret * 64) ^ secret) % 16_777_216;
    secret = ((secret / 32) ^ secret) % 16_777_216;
    ((secret * 2048) ^ secret) % 16_777_216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_calculate_number_after_time() -> miette::Result<()> {
        calculate_n_secrets(123, 10);
        let result = calculate_next(123);
        assert_eq!(15887950, result);
        Ok(())
    }
}
