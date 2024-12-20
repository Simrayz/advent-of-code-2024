use std::collections::{HashMap, HashSet};

use crate::Maze;
use glam::IVec2;

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

pub fn process(input: &str, threshold: usize) -> miette::Result<String> {
    let map = Maze::new(input);

    let mut visited = HashSet::from([map.start]);
    let mut path = Vec::new();
    let mut current = map.start;

    loop {
        let next: IVec2 = *DIRECTIONS
            .iter()
            .filter_map(|direction| {
                let next = *direction + current;
                match map
                    .path
                    .get(&next)
                    .is_some_and(|_| !visited.contains(&next))
                {
                    true => Some(next),
                    false => None,
                }
            })
            .collect::<Vec<IVec2>>()
            .first()
            .unwrap();

        path.push(next);
        visited.insert(next);

        if next == map.end {
            break;
        }

        current = next;
    }

    let shortcut_count = get_shortcuts_above_threshold(&path, threshold);

    Ok(shortcut_count.to_string())
}

fn get_shortcuts_above_threshold(path: &Vec<IVec2>, threshold: usize) -> usize {
    let index_map: HashMap<IVec2, usize> = path
        .iter()
        .enumerate()
        .map(|(idx, pos)| (*pos, idx))
        .collect();

    let shortcut_count = path
        .iter()
        .enumerate()
        .map(|(idx, pos)| {
            DIRECTIONS
                .iter()
                .filter(|direction| {
                    let next = *direction * 2 + pos;

                    match index_map.get(&next) {
                        Some(next_index) if next_index > &idx => {
                            (next_index - idx - 2) >= threshold
                        }
                        _ => false,
                    }
                })
                .count()
        })
        .sum();

    shortcut_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input-example.txt"), 50);
        assert_eq!("", result?);
        Ok(())
    }
}
