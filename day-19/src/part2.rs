use crate::parse_input;
use cached::proc_macro::cached;

pub fn process(input: &str) -> miette::Result<String> {
    let (towels, designs) = parse_input(input);

    let possible_designs: usize = designs
        .iter()
        .map(|design| validate_design(design, &towels))
        .sum();

    Ok(possible_designs.to_string())
}

#[cached(key = "String", convert = r##"{ format!("{design}") }"##)]
fn validate_design(design: &str, towels: &[&str]) -> usize {
    return towels
        .iter()
        .filter_map(|towel| {
            if design.starts_with(*towel) {
                let new_design = &design[towel.len()..];
                if new_design.is_empty() {
                    return Some(1);
                }
                Some(validate_design(new_design, towels))
            } else {
                None
            }
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input-example.txt"));
        assert_eq!("16", result?);
        Ok(())
    }
}
