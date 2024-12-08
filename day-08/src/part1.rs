use itertools::Itertools;
use std::collections::HashSet;

use crate::{add, subtract, Map};

pub fn process(input: &str, debug: bool) -> miette::Result<String> {
    let map = Map::new(input);

    let antinodes: HashSet<_> = map
        .antennas
        .values()
        .flat_map(|positions| {
            positions.iter().combinations(2).map(|pair| {
                let distance = map.get_distance(*pair[0], *pair[1]);
                vec![subtract(*pair[0], distance), add(*pair[1], distance)]
            })
        })
        .flatten()
        .filter(|node| map.within_bounds(*node))
        .collect();

    if debug {
        map.display_with_antinodes(&antinodes);
    }

    Ok(antinodes.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input1.txt"), true);
        assert_eq!("14", result?);
        Ok(())
    }
}
