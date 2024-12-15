use std::collections::HashMap;

use glam::IVec2;

use crate::robot::Board;

pub fn process(input: &str) -> miette::Result<String> {
    let board = Board::new(input);

    let mut position_map = HashMap::<IVec2, usize>::new();
    let mut quadrant_count_map = HashMap::<usize, usize>::new();

    for robot in &board.robots {
        let position = board.get_new_robot_position(&robot, 100);
        let quadrant = board.get_position_quadrant(position);

        if quadrant.is_some() {
            let quadrant = quadrant.unwrap();
            quadrant_count_map
                .entry(quadrant)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        position_map
            .entry(position)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let safety_factor: usize = quadrant_count_map.values().product();

    Ok(safety_factor.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input1.txt"));
        assert_eq!("12", result?);
        Ok(())
    }
}
