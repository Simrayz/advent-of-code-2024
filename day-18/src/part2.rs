use glam::IVec2;
use pathfinding::prelude::dijkstra;
use rayon::prelude::*;
use std::collections::HashSet;

use crate::{find_valid_successors, MemorySpace};

pub fn process(
    input: &str,
    board: (usize, usize),
    simulate_count: usize,
) -> miette::Result<String> {
    let space = MemorySpace::new(board.0, board.1, input);
    let start = IVec2::new(0, 0);
    let end = IVec2::new(space.width as i32, space.height as i32);

    let first_result = (simulate_count..space.corruptions.len())
        .into_par_iter()
        .find_map_first(|idx| {
            let corruptions = space.corruptions[..idx]
                .iter()
                .cloned()
                .collect::<HashSet<IVec2>>();

            match dijkstra(
                &start,
                |position: &IVec2| find_valid_successors(&space, &position, &corruptions),
                |&pos| pos == end,
            ) {
                Some(_) => None,
                None => Some(idx),
            }
        });

    let solution = match first_result {
        Some(solution) => space.corruptions[solution - 1],
        None => return Err(miette::miette!("No solution found")),
    };
    let formatted_solution = format!("{},{}", solution.x, solution.y);

    Ok(formatted_solution.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input-example.txt"), (6, 6), 12);
        assert_eq!("6,1", result?);
        Ok(())
    }
}
