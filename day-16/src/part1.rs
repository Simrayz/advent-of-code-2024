use glam::IVec2;
use pathfinding::prelude::dijkstra;

use crate::Maze;

pub fn process(input: &str) -> miette::Result<String> {
    let map = Maze::new(input);

    let result = dijkstra(
        &(map.start, IVec2::X),
        |(position, direction)| {
            let next_position = position + direction;
            if map.walls.contains(&next_position) {
                vec![
                    ((*position, direction.perp()), 1000),
                    ((*position, -direction.perp()), 1000),
                ]
            } else {
                vec![
                    ((next_position, *direction), 1),
                    ((*position, direction.perp()), 1000),
                    ((*position, -direction.perp()), 1000),
                ]
            }
        },
        |&(pos, _)| pos == map.end,
    )
    .expect("A valid map");

    Ok(result.1.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process_example1() -> miette::Result<()> {
        let result = process(include_str!("../input-example1.txt"));
        assert_eq!("7036", result?);
        Ok(())
    }

    #[test_log::test]
    fn test_process_example2() -> miette::Result<()> {
        let result = process(include_str!("../input-example2.txt"));
        assert_eq!("11048", result?);
        Ok(())
    }
}
