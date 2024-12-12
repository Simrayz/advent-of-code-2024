use std::collections::{HashMap, HashSet, VecDeque};

use glam::IVec2;

pub fn process(input: &str) -> miette::Result<String> {
    let garden = garden_map(input);
    let mut visited = HashSet::<IVec2>::new();

    let regions: Vec<_> = garden
        .keys()
        .filter_map(|start| find_plots(&garden, start, &mut visited))
        .collect();

    let region_sizes: usize = regions
        .iter()
        .map(|region| {
            let area = region.len();
            let perimeter: usize = region.iter().map(|(_, p)| p).sum();
            return area * perimeter;
        })
        .sum();

    Ok(region_sizes.to_string())
}

pub fn garden_map(input: &str) -> HashMap<IVec2, char> {
    input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, char)| (IVec2::new(row as i32, col as i32), char))
                .collect::<Vec<(IVec2, char)>>()
        })
        .flatten()
        .collect::<HashMap<IVec2, char>>()
}

pub fn find_plots(
    garden: &HashMap<IVec2, char>,
    start: &IVec2,
    visited: &mut HashSet<IVec2>,
) -> Option<Vec<(IVec2, usize)>> {
    let mut region: Vec<(IVec2, usize)> = Vec::new();
    let mut queue = VecDeque::from([*start]);

    if visited.contains(start) {
        return None;
    }

    loop {
        if queue.is_empty() {
            break;
        }
        let vertex = queue.pop_front().unwrap();

        visited.insert(vertex);

        let valid_neighbors = get_valid_neighbors(&garden, vertex);
        region.push((vertex, 4 - valid_neighbors.len()));

        let next_neighbors = valid_neighbors
            .into_iter()
            .filter(|x| !visited.contains(x))
            .collect::<Vec<IVec2>>();

        for neighbor in next_neighbors {
            queue.push_back(neighbor);
            visited.insert(neighbor);
        }
    }

    Some(region)
}

fn get_valid_neighbors(map: &HashMap<IVec2, char>, current_vertex: IVec2) -> Vec<IVec2> {
    let current_char = map.get(&current_vertex).unwrap();
    DIRECTIONS
        .iter()
        .map(|direction| current_vertex + direction)
        .filter(|position| map.contains_key(position) && map[position] == *current_char)
        .collect()
}

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process_input1() -> miette::Result<()> {
        let result = process(include_str!("../input1.txt"));
        assert_eq!("140", result?);
        Ok(())
    }

    #[test_log::test]
    fn test_process_input2() -> miette::Result<()> {
        let result = process(include_str!("../input2.txt"));
        assert_eq!("1930", result?);
        Ok(())
    }
}
