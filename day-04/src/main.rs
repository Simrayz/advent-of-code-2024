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

const X_DIRECTIONS: [[IVec2; 2]; 4] = [
    [IVec2::new(-1, -1), IVec2::new(1, 1)],
    [IVec2::new(-1, 1), IVec2::new(1, -1)],
    [IVec2::new(1, 1), IVec2::new(-1, -1)],
    [IVec2::new(1, -1), IVec2::new(-1, 1)],
];

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();

    const INPUT: &str = include_str!("../input1.txt");
    println!("XMAS count: {:?}", process_part1(INPUT));
    println!("Cross-MAS count: {:?}", process_part2(INPUT));
}

fn process_part1(input: &str) -> usize {
    let positions = create_position_map(input);
    return find_xmas_occurances(&positions);
}

fn process_part2(input: &str) -> usize {
    let positions = create_position_map(input);
    return find_cross_mas_occurances(&positions);
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

    const INPUT: &str = include_str!("../example1.txt");
    const INPUT_SMALL: &str = include_str!("../example2.txt");

    #[test]
    fn test_process_part1() {
        let result = process_part1(INPUT);
        assert_eq!(18, result);
    }

    #[test]
    fn test_process_part2() {
        let result = process_part2(INPUT);
        assert_eq!(9, result);
    }

    #[test]
    fn test_create_position_map() {
        let result = create_position_map(INPUT_SMALL);
        dbg!(result);
    }
}
