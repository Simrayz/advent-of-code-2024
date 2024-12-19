use glam::IVec2;
use pathfinding::prelude::dijkstra;
use std::collections::HashSet;

use crate::{find_valid_successors, MemorySpace};

pub fn process(input: &str, size: usize, simulate_count: usize) -> miette::Result<String> {
    let memory = MemorySpace::new(size, input);
    let start = IVec2::new(0, 0);
    let end = IVec2::new(size as i32, size as i32);

    let mut low = simulate_count;
    let mut high = memory.corruptions.len();

    while low < high {
        let mid = (low + high) / 2;
        let corruptions = memory.corruptions[..mid]
            .iter()
            .cloned()
            .collect::<HashSet<IVec2>>();

        let search = dijkstra(
            &start,
            |position: &IVec2| find_valid_successors(&memory, &position, &corruptions),
            |&pos| pos == end,
        );

        match search {
            Some(_) => low = mid + 1,
            None => high = mid,
        }
    }

    let solution = memory.corruptions[high - 1];
    let formatted_solution = format!("{},{}", solution.x, solution.y);

    Ok(formatted_solution.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input-example.txt"), 6, 12);
        assert_eq!("6,1", result?);
        Ok(())
    }
}
