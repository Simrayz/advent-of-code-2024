use itertools::Itertools;
use std::collections::HashSet;

use crate::{add, subtract, Map};

pub fn process(input: &str, debug: bool) -> miette::Result<String> {
    let map = Map::new(input);

    let antinodes: HashSet<_> = map
        .antennas
        .values()
        .flat_map(|positions| {
            let antenna_pairs = positions.iter().combinations(2);
            antenna_pairs.map(|pair| {
                let distance = map.get_distance(*pair[0], *pair[1]);
                let antinodes1 = get_antinodes(&map, *pair[0], distance, add, 1);
                let antinodes2 = get_antinodes(&map, *pair[1], distance, subtract, 1);

                [antinodes1, antinodes2].concat()
            })
        })
        .flatten()
        .filter(|node| map.within_bounds(*node))
        .collect();

    if debug {
        map.display_with_antinodes(&antinodes);
    }

    let sum = antinodes.len();
    Ok(sum.to_string())
}

fn get_antinodes(
    map: &Map,
    node: (i8, i8),
    distance: (i8, i8),
    func: fn((i8, i8), (i8, i8)) -> (i8, i8),
    n_factor: i8,
) -> Vec<(i8, i8)> {
    let mut results = Vec::new();
    let mut current_factor = n_factor;

    loop {
        let antinode = func(
            node,
            (distance.0 * current_factor, distance.1 * current_factor),
        );

        if !map.within_bounds(antinode) {
            break; // Terminate the loop if the antinode is out of bounds
        }

        results.push(antinode); // Add the valid antinode to the results
        current_factor += 1; // Increment the factor for the next iteration
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input1.txt"), true);
        assert_eq!("34", result?);
        Ok(())
    }
}
