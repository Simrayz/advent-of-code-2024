use std::collections::HashMap;

use glam::IVec2;

pub struct Board {
    pub width: i32,
    pub height: i32,
    pub robots: Vec<Robot>,
}

impl Default for Board {
    fn default() -> Self {
        let is_test = cfg!(test);
        Self {
            width: if is_test { 11 } else { 101 },
            height: if is_test { 7 } else { 103 },
            robots: Vec::new(),
        }
    }
}

impl Board {
    pub fn new(input: &str) -> Self {
        let robots: Vec<Robot> = input.lines().map(|line| Robot::from_input(line)).collect();

        Self {
            robots,
            ..Default::default()
        }
    }

    pub fn move_robot(&mut self, robot: &mut Robot, time: i32) {
        let new_position = self.get_new_robot_position(robot, time);
        robot.position = new_position;
    }

    pub fn get_new_robot_position(&self, robot: &Robot, time: i32) -> IVec2 {
        (robot.position + robot.direction * time).rem_euclid(IVec2::new(self.width, self.height))
    }

    pub fn get_position_quadrant(&self, position: IVec2) -> Option<usize> {
        let half_width = self.width / 2;
        let half_height = self.height / 2;

        let col = position.x;
        let row = position.y;

        if (col == half_width) || (row == half_height) {
            return None;
        }

        match (col < half_width, row < half_height) {
            (true, true) => Some(1),
            (false, true) => Some(2),
            (true, false) => Some(3),
            (false, false) => Some(4),
        }
    }

    pub fn display(&self, position_map: &HashMap<IVec2, usize>) {
        for row in 0..self.height {
            for col in 0..self.width {
                let position = IVec2::new(col, row);
                match position_map.get(&position) {
                    Some(num) => print!("{}", num),
                    _ => print!("."),
                }
            }
            println!();
        }
    }
}

#[derive(Debug)]
pub struct Robot {
    pub position: IVec2,
    pub direction: IVec2,
}

impl Robot {
    pub fn from_input(input: &str) -> Self {
        // p=0,4 v=3,-3
        let (position, direction) = input.trim().split_once(' ').unwrap();

        let position = position[2..]
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();
        let direction = direction[2..]
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();

        Self {
            position: IVec2::from_slice(&position),
            direction: IVec2::from_slice(&direction),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_new_robot() -> miette::Result<()> {
        let input = "p=0,4 v=3,-3";
        let robot = Robot::from_input(input);
        assert_eq!(robot.position, IVec2::new(0, 4));
        assert_eq!(robot.direction, IVec2::new(3, -3));
        Ok(())
    }

    #[test_log::test]
    fn test_move_in_time() -> miette::Result<()> {
        let input = "p=0,4 v=3,-3";
        let robot = Robot::from_input(input);
        let board = Board::default();
        let position = board.get_new_robot_position(&robot, 2);
        assert_eq!(position, IVec2::new(6, 5));
        Ok(())
    }
}
