use pathfinding::prelude::dijkstra;
use rayon::prelude::*;
use std::collections::HashMap;

use crate::Maze;
use glam::IVec2;

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

pub fn process(input: &str, threshold: usize) -> miette::Result<String> {
    let map = Maze::new(input);

    let (path, _non_cheat_cost) = dijkstra(
        &map.start,
        |position| {
            DIRECTIONS
                .iter()
                .filter_map(|direction| {
                    let next_position = direction + position;
                    map.path
                        .contains(&next_position)
                        .then_some((next_position, 1))
                })
                .collect::<Vec<_>>()
        },
        |&pos| pos == map.end,
    )
    .expect("A valid AoC path");

    let shortcut_count = get_shortcuts_above_threshold(&path, threshold);

    Ok(shortcut_count.to_string())
}

fn get_shortcuts_above_threshold(path: &Vec<IVec2>, threshold: usize) -> usize {
    let index_map: HashMap<IVec2, usize> = path
        .iter()
        .enumerate()
        .map(|(idx, pos)| (*pos, idx))
        .collect();

    let shortcut_count = path
        .par_iter()
        .enumerate()
        .map(|(idx, pos)| {
            DIRECTIONS
                .iter()
                .filter(|direction| {
                    let next = *direction * 2 + pos;

                    match index_map.get(&next) {
                        Some(next_index) if next_index > &idx => {
                            (next_index - idx - 2) >= threshold
                        }
                        _ => false,
                    }
                })
                .count()
        })
        .sum();

    shortcut_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input-example.txt"), 50);
        assert_eq!("", result?);
        Ok(())
    }
}
