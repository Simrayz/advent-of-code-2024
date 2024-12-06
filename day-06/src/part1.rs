use glam::IVec2;
use std::collections::HashSet;

use crate::board::Board;

pub fn process(input: &str) -> miette::Result<String> {
    let board = Board::new(input);

    let result = find_unique_positions(
        &board,
        board.start_position,
        IVec2::new(-1, 0),
        &mut HashSet::new(),
    );

    Ok(result.to_string())
}

fn find_unique_positions(
    board: &Board,
    current_position: IVec2,
    direction: IVec2,
    visited: &mut HashSet<IVec2>,
) -> usize {
    visited.insert(current_position);
    let next_position = current_position + direction;
    let next_direction = match board.positions.get(&next_position) {
        Some('#') => Some(clockwise_perpendicular(direction)),
        None => None,
        Some(_) => Some(direction),
    };

    match next_direction {
        None => return visited.len(),
        Some(direction) => {
            let new_position = current_position + direction;
            find_unique_positions(board, new_position, direction, visited)
        }
    }
}

fn clockwise_perpendicular(direction: IVec2) -> IVec2 {
    IVec2::new(direction.y, -direction.x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_rotate_direction() -> miette::Result<()> {
        let direction = IVec2::new(1, 0);
        let result = clockwise_perpendicular(direction);
        assert_eq!(IVec2::new(0, -1), result);
        Ok(())
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input1.txt"));
        assert_eq!("41", result?);
        Ok(())
    }
}
