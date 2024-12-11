use glam::IVec2;
use std::collections::HashMap;

use crate::search_trail_dfs;

pub fn process(input: &str) -> miette::Result<String> {
    let map = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    let height = c.to_digit(10).unwrap() as usize;
                    (IVec2::new(row as i32, col as i32), height)
                })
                .collect::<Vec<(IVec2, usize)>>()
        })
        .flatten()
        .collect::<HashMap<IVec2, usize>>();

    let unique: usize = map
        .iter()
        .filter(|(_, h)| *h == &0)
        .map(|(position, _h)| search_trail_dfs(&map, position, false).0)
        .sum();

    Ok(unique.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process_example1() -> miette::Result<()> {
        let result = process(include_str!("../input1.txt"));
        assert_eq!("1", result?);
        Ok(())
    }

    #[test_log::test]
    fn test_process_example2() -> miette::Result<()> {
        let result = process(include_str!("../input2.txt"));
        assert_eq!("36", result?);
        Ok(())
    }
}
