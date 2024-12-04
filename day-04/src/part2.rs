use glam::IVec2;
use std::collections::HashMap;

const X_DIRECTIONS: [[IVec2; 2]; 4] = [
    [IVec2::new(-1, -1), IVec2::new(1, 1)],
    [IVec2::new(-1, 1), IVec2::new(1, -1)],
    [IVec2::new(1, 1), IVec2::new(-1, -1)],
    [IVec2::new(1, -1), IVec2::new(-1, 1)],
];

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let positions = create_position_map(input);
    let result = find_cross_mas_occurances(&positions);

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

fn find_cross_mas_occurances(positions: &HashMap<IVec2, char>) -> usize {
    let ms = ['M', 'S'];
    positions
        .iter()
        .filter(|(_, value)| **value == 'A')
        .filter(|(key, _value)| {
            let mas_count = X_DIRECTIONS
                .iter()
                .map(|direction| {
                    let letters = direction
                        .iter()
                        .map(|dir| positions.get(&(*key + *dir)))
                        .enumerate()
                        .all(|(i, value)| ms.get(i) == value);

                    return letters;
                })
                .filter(|b| *b)
                .count();

            return mas_count == 2;
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input1.txt");

    #[test_log::test]
    fn test_process_part2() -> miette::Result<()> {
        let result = process(INPUT);
        assert_eq!("9", result?);
        Ok(())
    }
}
