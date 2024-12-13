use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn process(input: &str) -> miette::Result<String> {
    let garden = garden_map(input);
    let mut visited = HashSet::<(i32, i32)>::new();

    let regions: Vec<_> = garden
        .keys()
        .filter_map(|start| find_plots(&garden, start, &mut visited))
        .collect();

    let perimeter_price: i32 = regions
        .iter()
        .map(|region| get_region_price(&garden, region))
        .sum();

    Ok(perimeter_price.to_string())
}

pub fn garden_map(input: &str) -> HashMap<(i32, i32), char> {
    input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, char)| ((row as i32, col as i32), char))
                .collect::<Vec<((i32, i32), char)>>()
        })
        .flatten()
        .collect::<HashMap<(i32, i32), char>>()
}

pub fn find_plots(
    garden: &HashMap<(i32, i32), char>,
    start: &(i32, i32),
    visited: &mut HashSet<(i32, i32)>,
) -> Option<Vec<(i32, i32)>> {
    let mut region: Vec<(i32, i32)> = Vec::new();
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
        region.push(vertex);

        let next_neighbors = valid_neighbors
            .into_iter()
            .filter(|x| !visited.contains(x))
            .collect::<Vec<(i32, i32)>>();

        for neighbor in next_neighbors {
            queue.push_back(neighbor);
            visited.insert(neighbor);
        }
    }

    Some(region)
}

const DIRECTIONS: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

fn get_valid_neighbors(
    map: &HashMap<(i32, i32), char>,
    current_vertex: (i32, i32),
) -> Vec<(i32, i32)> {
    let current_char = map.get(&current_vertex).unwrap();
    DIRECTIONS
        .iter()
        .map(|[x2, y2]| (current_vertex.0 + x2, current_vertex.1 + y2))
        .filter(|position| map.contains_key(position) && map[position] == *current_char)
        .collect()
}

fn get_region_price(garden: &HashMap<(i32, i32), char>, region: &Vec<(i32, i32)>) -> i32 {
    let group_id = garden.get(&region[0]).unwrap();
    let mut count = 0;

    region.iter().for_each(|node| {
        let direction_iterator = DIRECTIONS.iter().circular_tuple_windows::<(_, _)>();
        for ([x, y], [x1, y1]) in direction_iterator {
            let neighbor1 = garden
                .get(&(x + node.0, y + node.1))
                .is_some_and(|c| c == group_id);
            let neighbor2 = garden
                .get(&(x1 + node.0, y1 + node.1))
                .is_some_and(|c| c == group_id);

            if !neighbor1 && !neighbor2 {
                count += 1;
            }
            if neighbor1
                && neighbor2
                && garden
                    .get(&(x + x1 + node.0, y + y1 + node.1))
                    .is_some_and(|char| char != group_id)
            {
                count += 1;
            }
        }
    });

    count * (region.len() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process_input1() -> miette::Result<()> {
        let result = process(include_str!("../input1.txt"));
        assert_eq!("80", result?);
        Ok(())
    }

    #[test_log::test]
    fn test_process_input_e() -> miette::Result<()> {
        let result = process(include_str!("../input_e.txt"));
        assert_eq!("236", result?);
        Ok(())
    }

    #[test_log::test]
    fn test_process_input_ab() -> miette::Result<()> {
        let result = process(include_str!("../input_ab.txt"));
        assert_eq!("368", result?);
        Ok(())
    }
}
