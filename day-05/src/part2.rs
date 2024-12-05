use crate::parser::Solver;

pub fn process(input: &str) -> miette::Result<String> {
    let solver = Solver::new(input);

    let (_, invalid) = solver.validate_updates();

    let fixed = invalid
        .iter()
        .map(|update| {
            let mut fixed_update = (*update).clone();
            fixed_update.sort_by(|a, b| {
                if solver.rules.get(a).is_some_and(|pages| pages.contains(b)) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
            fixed_update
        })
        .collect::<Vec<_>>();

    let result: usize = fixed
        .iter()
        .map(|update| {
            let middle_index = update.len() / 2;
            update[middle_index]
        })
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input1.txt");

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(INPUT);
        assert_eq!("123", result?);
        Ok(())
    }
}
