use itertools::Itertools;
use pathfinding::prelude::dijkstra;

use crate::Maze;
use glam::IVec2;

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

pub fn process(input: &str, threshold: usize) -> miette::Result<String> {
    let map = Maze::new(input);

    let (path, non_cheat_cost) = dijkstra(
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

    let result = path
        .iter()
        .enumerate()
        // Get the cost of each pair of positions in the path
        .tuple_combinations()
        .filter_map(|((start_cost, start), (end_cost, end))| {
            let distance = (start - end).abs().element_sum() as usize;
            if distance > 20 {
                return None;
            }
            let cheat_cost = start_cost + distance + non_cheat_cost - end_cost;
            Some(non_cheat_cost - cheat_cost)
        })
        .filter(|cost| cost >= &threshold)
        .count();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input-example.txt"), 50);
        assert_eq!("285", result?);
        Ok(())
    }
}
