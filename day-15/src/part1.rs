use std::collections::HashMap;

use glam::IVec2;

pub fn process(input: &str) -> miette::Result<String> {
    let mut board = Board::new(input);
    let mut current_position = board.start_position;

    for direction in board.moves.iter() {
        let direction_vec = direction_to_move(direction);
        let mut next = current_position + direction_vec;

        while board.map.get(&next) == Some(&'O') {
            next += direction_vec;
        }

        if board.map.get(&next) == Some(&'.') {
            while next != current_position {
                let previous = next - direction_vec;
                let previous_char = board.map.get(&previous).unwrap();
                board.map.insert(next, *previous_char);

                next = previous;
            }

            board.map.insert(current_position, '.');
            current_position += direction_vec;
        }
    }

    // board.display();

    let result = board.map.iter().fold(0, |mut acc, (pos, value)| {
        if value == &'O' {
            acc += pos.y * 100 + pos.x
        }
        acc
    });

    Ok(result.to_string())
}

fn direction_to_move(direction: &char) -> IVec2 {
    match direction {
        '^' => IVec2::new(0, -1),
        'v' => IVec2::new(0, 1),
        '>' => IVec2::new(1, 0),
        '<' => IVec2::new(-1, 0),
        _ => panic!("Unknown direction"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input-example1.txt"));
        assert_eq!("2028", result?);
        Ok(())
    }

    #[test_log::test]
    fn test_process_input1_large() -> miette::Result<()> {
        let result = process(include_str!("../input-example2.txt"));
        assert_eq!("10092", result?);
        Ok(())
    }
}

#[derive(Debug)]
struct Board {
    map: HashMap<IVec2, char>,
    moves: Vec<char>,
    start_position: IVec2,
    size: (usize, usize),
}

impl Board {
    fn new(input: &str) -> Self {
        let (board_input, moves) = input.split_once("\n\n").unwrap();

        let rows = board_input.lines().count();
        let cols = board_input.lines().next().unwrap().len();

        let size = (rows, cols);
        let mut start_position = IVec2::new(0, 0);

        let board: HashMap<IVec2, char> = board_input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| {
                        let position = IVec2::new(col as i32, row as i32);
                        if c == '@' {
                            start_position = position;
                        }
                        (position, c)
                    })
                    .collect::<Vec<(IVec2, char)>>()
            })
            .flatten()
            .collect();

        Self {
            map: board,
            moves: moves.replace("\n", "").chars().collect(),
            start_position,
            size,
        }
    }

    fn display(&self) {
        for row in 0..self.size.0 {
            for col in 0..self.size.1 {
                print!("{}", self.map[&IVec2::new(col as i32, row as i32)]);
            }
            println!();
        }
    }
}
