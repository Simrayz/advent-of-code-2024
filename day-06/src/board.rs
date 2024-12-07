use std::iter;

use fxhash::{FxHashMap, FxHashSet};
use glam::I16Vec2;

#[derive(Debug)]
pub struct Board {
    pub start_position: I16Vec2,
    pub positions: FxHashMap<I16Vec2, char>,
}

impl Board {
    pub fn new(input: &str) -> Self {
        let mut positions = FxHashMap::<I16Vec2, char>::default();
        let mut start_position = I16Vec2::new(0, 0);

        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                positions.insert(I16Vec2::new(row as i16, col as i16), c);
                if c == '^' {
                    start_position = I16Vec2::new(row as i16, col as i16);
                }
            }
        }

        Self {
            start_position,
            positions,
        }
    }
    pub fn find_unique_positions(&self) -> FxHashSet<I16Vec2> {
        let positions = find_unique_positions(self.start_position, &self.positions);

        return positions;
    }

    pub fn find_unique_positions_count(&self) -> usize {
        self.find_unique_positions().len()
    }
}

fn find_unique_positions(
    start_position: I16Vec2,
    positions: &FxHashMap<I16Vec2, char>,
) -> FxHashSet<I16Vec2> {
    let mut current_position = start_position;
    let mut visited = FxHashSet::from_iter(iter::once(current_position));
    let mut direction = I16Vec2::new(-1, 0);

    loop {
        visited.insert(current_position);
        let next_position = current_position + direction;
        let next_direction = match positions.get(&next_position) {
            Some('#') => Some(rotate_direction(direction)),
            None => None,
            Some(_) => Some(direction),
        };

        match next_direction {
            None => break,
            Some(new_direction) => {
                let new_position = current_position + new_direction;
                current_position = new_position;
                direction = new_direction;
            }
        }
    }

    return visited;
}

pub fn rotate_direction(direction: I16Vec2) -> I16Vec2 {
    I16Vec2::new(direction.y, -direction.x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_rotate_direction() -> miette::Result<()> {
        let direction = I16Vec2::new(1, 0);
        let result = rotate_direction(direction);
        assert_eq!(I16Vec2::new(0, -1), result);
        Ok(())
    }
}
