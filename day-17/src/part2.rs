use std::collections::VecDeque;

use crate::Computer;

pub fn process(input: &str, debug: bool) -> miette::Result<String> {
    let initial_cpu = Computer::from_input(input);

    let expected = &initial_cpu.program;
    let mut queue = VecDeque::<(usize, usize)>::new();
    queue.push_back((0, 1));

    while let Some((test_a, digits)) = queue.pop_front() {
        for i in 0..=7 {
            let next_a = (test_a << 3) + i;
            let mut cpu = initial_cpu.copy_with_a(next_a);
            let output = cpu.run();

            if output.ends_with(&expected[expected.len() - digits..]) {
                if debug {
                    println!(
                        "i={}, A={}, Next = {}, Match={:?}",
                        i,
                        test_a,
                        next_a,
                        &expected[expected.len() - digits..]
                    );
                }
                if digits == expected.len() {
                    return Ok(next_a.to_string());
                }

                queue.push_back((next_a, digits + 1));
            }
        }
    }

    Ok("-1".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input-example2.txt"), false);
        assert_eq!("117440", result?);
        Ok(())
    }
}
