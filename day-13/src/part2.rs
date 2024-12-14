use glam::U64Vec2;

use crate::machine::*;

const A_COST: u64 = 3;
const B_COST: u64 = 1;

pub fn process(input: &str) -> miette::Result<String> {
    let machines = parse_claw_machines(input, true);

    let sum: u64 = machines
        .iter()
        .filter_map(|machine| {
            machine
                .solve()
                .map(|value| (value * U64Vec2::new(A_COST, B_COST)).element_sum())
                .ok()
        })
        .sum();
    Ok(sum.to_string())
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
