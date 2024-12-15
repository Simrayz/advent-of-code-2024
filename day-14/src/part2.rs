use crate::robot::Board;
use fxhash::FxHashSet;
use glam::IVec2;
use std::collections::HashMap;

pub fn process(input: &str, debug: bool) -> miette::Result<String> {
    let board = Board::new(input);

    let mut time = 0;
    let mut christmas_map = HashMap::<IVec2, usize>::new();

    loop {
        let mut positions = FxHashSet::<IVec2>::default();

        for robot in board.robots.iter() {
            let new_position = board.get_new_robot_position(robot, time);
            if positions.contains(&new_position) {
                break;
            }

            positions.insert(new_position);
        }

        if positions.len() == board.robots.len() {
            if debug {
                for position in positions.iter() {
                    christmas_map.insert(*position, 1);
                }
            }
            break;
        }

        time += 1;
    }

    if debug {
        board.display(&christmas_map);
    }

    Ok(time.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input1.txt"), true);
        assert_eq!("1", result?);
        Ok(())
    }
}
