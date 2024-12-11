use std::collections::{HashMap, HashSet, VecDeque};

use glam::IVec2;

pub mod part1;
pub mod part2;

pub const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

pub fn search_trail_dfs(
    map: &HashMap<IVec2, usize>,
    start_pos: &IVec2,
    collect: bool,
) -> (usize, Vec<IVec2>) {
    let mut visited: HashSet<IVec2> = HashSet::from([]);
    let mut queue: VecDeque<IVec2> = VecDeque::from([*start_pos]);

    loop {
        if queue.is_empty() {
            break;
        }

        let vertex = queue.pop_front().unwrap();
        let neighbors = get_valid_neighbors(map, vertex)
            .into_iter()
            .filter(|x| !visited.contains(x))
            .collect::<Vec<IVec2>>();

        for neighbor in neighbors {
            queue.push_back(neighbor);
            visited.insert(neighbor);
        }
    }
    let visited_iterator = visited.iter().filter(|pos| map.get(pos).unwrap() == &9);

    if collect {
        let visited_vec: Vec<IVec2> = visited_iterator.cloned().collect();
        (visited_vec.len(), visited_vec)
    } else {
        (visited_iterator.count(), Vec::new())
    }
}

pub fn get_valid_neighbors(map: &HashMap<IVec2, usize>, current_vertex: IVec2) -> Vec<IVec2> {
    DIRECTIONS
        .iter()
        .filter_map(|direction| {
            let new_vertex = direction + current_vertex as IVec2;

            let include = map.get(&new_vertex).is_some_and(|height| {
                let current_height = map.get(&current_vertex).unwrap();
                *height == *current_height + 1
            });

            if include {
                Some(new_vertex)
            } else {
                None
            }
        })
        .collect()
}
