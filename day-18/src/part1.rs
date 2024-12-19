use glam::IVec2;
use pathfinding::prelude::dijkstra;

use crate::MemorySpace;

pub fn process(input: &str, size: usize, simulate_count: usize) -> miette::Result<String> {
    let mut space = MemorySpace::new(size, input);
    let start = IVec2::new(0, 0);
    let end = IVec2::new(space.size as i32, space.size as i32);

    space.simulate(simulate_count);

    let result = dijkstra(
        &start,
        |position| space.find_valid_successors(*position),
        |&pos| pos == end,
    )
    .expect("A valid map");

    Ok(result.1.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input-example.txt"), 6, 12);
        assert_eq!("22", result?);
        Ok(())
    }
}
