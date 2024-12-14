use crate::machine::*;
use glam::U64Vec2;

pub fn process(input: &str) -> miette::Result<String> {
    let machines = parse_claw_machines(input, false);
    let total_cost: u64 = machines
        .iter()
        .filter_map(|machine| {
            machine
                .solve()
                .map(|value| (value * U64Vec2::new(3, 1)).element_sum())
                .ok()
        })
        .sum();
    Ok(total_cost.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input1.txt"));
        assert_eq!("480", result?);
        Ok(())
    }
}
