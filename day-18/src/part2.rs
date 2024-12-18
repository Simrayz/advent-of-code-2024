use glam::IVec2;
use pathfinding::prelude::dijkstra;
use rayon::prelude::*;
use std::collections::HashSet;

use crate::MemorySpace;

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
            let succesors =
                |position: &IVec2| find_valid_successors(&space, &position, &corruptions);

            match dijkstra(&start, succesors, |&pos| pos == end) {
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

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];
fn find_valid_successors(
    space: &MemorySpace,
    position: &IVec2,
    collisions: &HashSet<IVec2>,
) -> Vec<(IVec2, usize)> {
    DIRECTIONS
        .iter()
        .filter_map(|direction| {
            let next_position = position + direction;
            let out_of_bounds = next_position.x < 0
                || next_position.y < 0
                || next_position.x > space.width as i32
                || next_position.y > space.height as i32;

            if collisions.contains(&next_position) || out_of_bounds {
                None
            } else {
                Some((next_position, 1))
            }
        })
        .collect()
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
