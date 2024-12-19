use cached::proc_macro::cached;

use crate::parse_input;

pub fn process(input: &str) -> miette::Result<String> {
    let (towels, designs) = parse_input(input);

    let possible_designs: usize = designs
        .iter()
        .filter(|design| validate_design(design, &towels))
        .count();

    Ok(possible_designs.to_string())
}

#[cached(key = "String", convert = r##"{ format!("{design}") }"##)]
fn validate_design(design: &str, towels: &[&str]) -> bool {
    return towels
        .iter()
        .map(|towel| {
            if design.starts_with(*towel) {
                let new_design = &design[towel.len()..];
                if new_design.is_empty() {
                    return true;
                }
                validate_design(new_design, towels)
            } else {
                false
            }
        })
        .any(|v| v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input-example.txt"));
        assert_eq!("6", result?);
        Ok(())
    }
}
