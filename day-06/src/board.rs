use std::collections::{HashMap, HashSet};

use glam::IVec2;

#[derive(Debug)]
pub struct Board {
    pub board: Vec<Vec<char>>,
    pub start_position: IVec2,
    pub positions: HashMap<IVec2, char>,
}

impl Board {
    pub fn new(input: &str) -> Self {
        let mut positions = HashMap::<IVec2, char>::new();
        let mut start_position = IVec2::new(0, 0);
        let board: Vec<Vec<char>> = input
            .lines()
            .into_iter()
            .enumerate()
            .map(|(row, line)| {
                let line = line
                    .chars()
                    .enumerate()
                    .filter_map(|(col, c)| {
                        positions.insert(IVec2::new(row as i32, col as i32), c);
                        if c == '^' {
                            start_position = IVec2::new(row as i32, col as i32);
                            return Some('X');
                        }
                        Some(c)
                    })
                    .collect::<Vec<char>>();
                line
            })
            .collect::<Vec<Vec<char>>>();

        Self {
            board,
            start_position,
            positions,
        }
    }
    pub fn find_unique_positions(&self) -> HashSet<IVec2> {
        let positions = find_unique_positions(self.start_position, &self.positions);

        return positions;
    }

    pub fn find_unique_positions_count(&self) -> usize {
        self.find_unique_positions().len()
    }
}

fn find_unique_positions(
    start_position: IVec2,
    positions: &HashMap<IVec2, char>,
) -> HashSet<IVec2> {
    let mut current_position = start_position;
    let mut visited = HashSet::<IVec2>::new();
    let mut direction = IVec2::new(-1, 0);

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

pub fn rotate_direction(direction: IVec2) -> IVec2 {
    IVec2::new(direction.y, -direction.x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_rotate_direction() -> miette::Result<()> {
        let direction = IVec2::new(1, 0);
        let result = rotate_direction(direction);
        assert_eq!(IVec2::new(0, -1), result);
        Ok(())
    }
}
