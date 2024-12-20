use std::collections::HashSet;

use glam::IVec2;

pub mod part1;
pub mod part2;

#[derive(Debug)]
struct Maze {
    start: IVec2,
    end: IVec2,
    path: HashSet<IVec2>,
    width: usize,
    height: usize,
}

impl Maze {
    fn new(input: &str) -> Self {
        let mut start = IVec2::new(0, 0);
        let mut end = IVec2::new(0, 0);
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();
        let path = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(col, c)| {
                        let position = IVec2::new(col as i32, row as i32);
                        match c {
                            'S' => {
                                start = position;
                                Some(position)
                            }
                            'E' => {
                                end = position;
                                Some(position)
                            }
                            '.' => Some(position),
                            _ => None,
                        }
                    })
                    .collect::<Vec<IVec2>>()
            })
            .flatten()
            .collect();

        Self {
            start,
            end,
            width,
            height,
            path,
        }
    }
}
