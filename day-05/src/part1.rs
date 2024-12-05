use crate::parser::Solver;

pub fn process(input: &str) -> miette::Result<String> {
    let solver = Solver::new(input);

    let (valid, _) = solver.validate_updates();
    let middle_indices: usize = solver.get_middle_page_sum(&valid);

    Ok(middle_indices.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input1.txt");

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(INPUT);
        assert_eq!("143", result?);
        Ok(())
    }
}
