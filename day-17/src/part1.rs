use crate::Computer;

pub fn process(input: &str) -> miette::Result<String> {
    let mut program = Computer::from_input(input);

    while program.pointer < program.program.len() {
        program.perform_operation();
    }

    Ok(program.to_output())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input-example.txt"));
        assert_eq!("4,6,3,5,6,3,5,2,1,0", result?);
        Ok(())
    }
}
