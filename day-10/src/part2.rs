use glam::IVec2;
use pathfinding::prelude::count_paths;
use std::collections::HashMap;

use crate::{get_valid_neighbors, search_trail_dfs};

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

    let unique_paths_count: usize = map
        .iter()
        .filter(|(_, h)| *h == &0)
        .map(|(position, _)| (position, search_trail_dfs(&map, position, true).1))
        .map(|(position, targets)| (rate_trail(&map, position, &targets)))
        .sum();

    Ok(unique_paths_count.to_string())
}

fn rate_trail(map: &HashMap<IVec2, usize>, start: &IVec2, targets: &Vec<IVec2>) -> usize {
    targets
        .iter()
        .map(|target| {
            count_paths(
                *start,
                |pos| get_valid_neighbors(map, *pos),
                |pos| pos == target,
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process_example1() -> miette::Result<()> {
        let result = process(include_str!("../input1.txt"));
        assert_eq!("16", result?);
        Ok(())
    }

    #[test_log::test]
    fn test_process_example2() -> miette::Result<()> {
        let result = process(include_str!("../input2.txt"));
        assert_eq!("81", result?);
        Ok(())
    }
}
