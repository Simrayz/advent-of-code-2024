use fxhash::FxHashSet;
use glam::I16Vec2;
use rayon::prelude::*;

use crate::board::{rotate_direction, Board};

pub fn process(input: &str) -> miette::Result<String> {
    let board = Board::new(input);

    let mut visited_positions: FxHashSet<_> = board.find_unique_positions();
    visited_positions.remove(&board.start_position);

    let wall_positions: FxHashSet<_> = board
        .positions
        .iter()
        .filter_map(|(pos, value)| match value {
            '#' => Some(*pos),
            _ => None,
        })
        .collect();

    let loop_count = visited_positions
        .par_iter()
        .filter(|new_wall_position| {
            let mut current_position = board.start_position;
            let mut direction = I16Vec2::new(-1, 0);
            let mut visited_positions =
                FxHashSet::from_iter([(board.start_position, direction.clone())]);

            loop {
                let next_position = current_position + direction;

                if wall_positions.contains(&next_position) || **new_wall_position == next_position {
                    direction = rotate_direction(direction);
                    continue;
                }

                // If there is a loop, you have encountered this before
                if visited_positions.get(&(next_position, direction)).is_some() {
                    break true;
                }

                match board.positions.get(&next_position) {
                    Some(_) => {
                        current_position = current_position + direction;
                        visited_positions.insert((current_position, direction));
                    }
                    _ => break false,
                }
            }
        })
        .count();

    Ok(loop_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input1.txt"));
        assert_eq!("6", result?);
        Ok(())
    }
}
