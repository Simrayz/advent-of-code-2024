use glam::IVec2;
use pathfinding::prelude::dijkstra;

use crate::MemorySpace;

pub fn process(
    input: &str,
    board: (usize, usize),
    simulate_count: usize,
) -> miette::Result<String> {
    let mut space = MemorySpace::new(board.0, board.1, input);
    let start = IVec2::new(0, 0);
    let end = IVec2::new(space.width as i32, space.height as i32);

    space.simulate(simulate_count);
    let mut solution = IVec2::new(-1, -1);

    for idx in simulate_count..space.corruptions.len() {
        space.add_corruption(space.corruptions[idx]);

        let result = dijkstra(
            &start,
            |position| space.find_valid_successors(*position),
            |&pos| pos == end,
        );
        if result.is_none() {
            solution = space.corruptions[idx];
            break;
        }
    }
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
