use std::collections::HashSet;

use glam::IVec2;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Maze {
    walls: HashSet<IVec2>,
    start: IVec2,
    end: IVec2,
    size: IVec2,
}

impl Maze {
    fn new(input: &str) -> Self {
        let mut start_node = IVec2::new(0, 0);
        let mut end_node = IVec2::new(0, 0);

        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();

        let walls: HashSet<IVec2> = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(col, c)| match c {
                        '#' => Some(IVec2::new(col as i32, row as i32)),
                        'S' => {
                            start_node = IVec2::new(col as i32, row as i32);
                            None
                        }
                        'E' => {
                            end_node = IVec2::new(col as i32, row as i32);
                            None
                        }
                        _ => None,
                    })
                    .collect::<Vec<IVec2>>()
            })
            .flatten()
            .collect();

        Maze {
            walls,
            start: start_node,
            end: end_node,
            size: IVec2::new(width as i32, height as i32),
        }
    }
    pub fn display(&self) {
        let mut output = String::new();
        for row in 0..self.size.y {
            for col in 0..self.size.x {
                let symbol = match IVec2::new(col as i32, row as i32) {
                    val if val == self.start => 'S',
                    val if val == self.end => 'E',
                    val if self.walls.contains(&val) => '#',
                    _ => '.',
                };
                output.push(symbol);
            }
            output.push('\n');
        }
        print!("{}", output);
    }
}
