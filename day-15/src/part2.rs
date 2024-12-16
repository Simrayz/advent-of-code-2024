use std::collections::HashSet;

use glam::IVec2;

pub fn process(input: &str) -> miette::Result<String> {
    let mut board = Board::new(input);

    let mut backup = board.map.clone();
    let mut current_position = board.start_position;

    for direction in board.moves.iter() {
        let direction_vec = direction_to_move(*direction);
        if let Some(visited) = expand_in_direction(&board, current_position, direction_vec) {
            // Copy visited positions to backup
            for position in visited.iter() {
                backup[position.y as usize][position.x as usize] =
                    board.map[position.y as usize][position.x as usize];
            }
            // Clear positions from the map
            for position in visited.iter() {
                board.map[position.y as usize][position.x as usize] = '.';
            }
            // Move backup positions into grid
            for position in visited.iter() {
                let next_position = *position + direction_vec;
                board.map[next_position.y as usize][next_position.x as usize] =
                    backup[position.y as usize][position.x as usize];
            }
            current_position += direction_vec;
        }
    }

    board.display();

    let result: usize = board
        .map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &c)| if c == '[' { 100 * y + x } else { 0 })
        })
        .sum();

    Ok(result.to_string())
}

fn expand_in_direction(board: &Board, position: IVec2, direction: IVec2) -> Option<HashSet<IVec2>> {
    let mut queue = Vec::from([position]);
    let mut visited = HashSet::from([position]);

    let left_wards = direction_to_move('<');
    let right_wards = direction_to_move('>');

    while let Some(new_position) = queue.pop() {
        let new_position = new_position + direction;
        if visited.contains(&new_position) {
            continue;
        }
        let next_char = board.map[new_position.y as usize][new_position.x as usize];
        match next_char {
            '.' => {}
            ']' | '[' => {
                queue.push(new_position);
                visited.insert(new_position);

                if direction.y != 0 {
                    let neighbor_direction = if next_char == ']' {
                        left_wards
                    } else {
                        right_wards
                    };
                    let neighbor = new_position + neighbor_direction;
                    queue.push(neighbor);
                    visited.insert(neighbor);
                }
            }
            _ => return None,
        }
    }

    Some(visited)
}

fn direction_to_move(direction: char) -> IVec2 {
    match direction {
        '^' => IVec2::new(0, -1),
        'v' => IVec2::new(0, 1),
        '>' => IVec2::new(1, 0),
        '<' => IVec2::new(-1, 0),
        _ => panic!("Unknown direction"),
    }
}

#[derive(Debug)]
struct Board {
    map: Vec<Vec<char>>,
    moves: Vec<char>,
    start_position: IVec2,
}

impl Board {
    fn new(input: &str) -> Self {
        let (board_input, moves) = input.split_once("\n\n").unwrap();

        let board: Vec<_> = board_input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| match char {
                        '.' => ['.', '.'],
                        '#' => ['#', '#'],
                        'O' => ['[', ']'],
                        '@' => ['@', '.'],
                        _ => panic!("Unknown char"),
                    })
                    .flatten()
                    .collect::<Vec<char>>()
            })
            .collect();

        let mut start_position = IVec2::ZERO;
        for y in 0..board.len() {
            for x in 0..board[y].len() {
                if board[y][x] == '@' {
                    start_position = IVec2::new(x as i32, y as i32);
                }
            }
        }

        Self {
            map: board,
            moves: moves.replace("\n", "").chars().collect(),
            start_position,
        }
    }

    fn display(&self) {
        for i in 0..self.map.len() {
            for j in 0..self.map[i].len() {
                print!("{}", self.map[i][j]);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process_input1_large() -> miette::Result<()> {
        let result = process(include_str!("../input-example2.txt"));
        assert_eq!("9021", result?);
        Ok(())
    }
}
