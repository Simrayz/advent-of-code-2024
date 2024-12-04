use glam::IVec2;
use std::collections::HashMap;

const DIRECTIONS: [[IVec2; 3]; 8] = [
    [IVec2::new(0, 1), IVec2::new(0, 2), IVec2::new(0, 3)], // east
    [IVec2::new(0, -1), IVec2::new(0, -2), IVec2::new(0, -3)], // west
    [IVec2::new(1, 1), IVec2::new(2, 2), IVec2::new(3, 3)], // south east
    [IVec2::new(1, -1), IVec2::new(2, -2), IVec2::new(3, -3)], // north east
    [IVec2::new(-1, 1), IVec2::new(-2, 2), IVec2::new(-3, 3)], // south west
    [IVec2::new(-1, -1), IVec2::new(-2, -2), IVec2::new(-3, -3)], // north west
    [IVec2::new(1, 0), IVec2::new(2, 0), IVec2::new(3, 0)], // south
    [IVec2::new(-1, 0), IVec2::new(-2, 0), IVec2::new(-3, 0)], // north
];

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let positions = create_position_map(input);
    let result = find_xmas_occurances(&positions);

    return Ok(result.to_string());
}

fn create_position_map(input: &str) -> HashMap<IVec2, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, value)| (IVec2::new(x as i32, y as i32), value))
        })
        .collect::<HashMap<IVec2, char>>()
}

fn find_xmas_occurances(positions: &HashMap<IVec2, char>) -> usize {
    let mas = ['M', 'A', 'S'];

    positions
        .iter()
        .filter(|(_, value)| **value == 'X')
        .map(|(key, _value)| {
            return DIRECTIONS
                .iter()
                .map(|direction| {
                    let letters = direction
                        .iter()
                        .map(|dir| positions.get(&(key + *dir)))
                        .enumerate()
                        .all(|(i, letter)| mas.get(i) == letter);

                    return letters;
                })
                .filter(|b| *b)
                .count();
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input1.txt");

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(INPUT);
        assert_eq!("18", result?);
        Ok(())
    }
}
