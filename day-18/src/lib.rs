use std::collections::{HashMap, HashSet};

use glam::IVec2;

pub mod part1;
pub mod part2;

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

pub struct MemorySpace {
    width: usize,
    height: usize,
    corruptions: Vec<IVec2>,
    fallen: HashSet<IVec2>,
}

impl MemorySpace {
    fn new(width: usize, height: usize, input: &str) -> Self {
        let corruptions: Vec<IVec2> = input
            .lines()
            .map(|line| {
                let pieces = line
                    .split(",")
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>();
                IVec2::new(pieces[0], pieces[1])
            })
            .collect();

        Self {
            width,
            height,
            corruptions,
            fallen: HashSet::new(),
        }
    }

    fn simulate(&mut self, num_bytes: usize) {
        for idx in 0..num_bytes {
            let current = self.corruptions[idx];
            self.fallen.insert(current);
        }
    }

    fn find_valid_successors(&self, position: IVec2) -> Vec<(IVec2, usize)> {
        DIRECTIONS
            .iter()
            .filter_map(|direction| {
                let next_position = position + direction;
                let out_of_bounds = next_position.x < 0
                    || next_position.y < 0
                    || next_position.x > self.width as i32
                    || next_position.y > self.height as i32;

                if self.fallen.contains(&next_position) || out_of_bounds {
                    None
                } else {
                    Some((next_position, 1))
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation() {
        let collissions: HashSet<IVec2> = HashSet::from(
            [
                (5, 4),
                (4, 2),
                (4, 5),
                (3, 0),
                (2, 1),
                (6, 3),
                (2, 4),
                (1, 5),
                (0, 6),
                (3, 3),
                (2, 6),
                (5, 1),
            ]
            .map(|(x, y)| IVec2::new(x, y)),
        );
        let input: &str = include_str!("../input-example.txt");

        let mut space = MemorySpace::new(7, 7, input);
        space.simulate(12);

        assert_eq!(space.fallen, collissions);
    }
}
