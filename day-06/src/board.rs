use std::collections::HashMap;

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
}
