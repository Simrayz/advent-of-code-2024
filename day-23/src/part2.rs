use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn process(input: &str) -> miette::Result<String> {
    let graph = build_graph(input);

    let combination_vertices: Vec<_> = graph
        .keys()
        .combinations(2)
        .filter_map(|pair| {
            let first = *pair[0];
            let second = *pair[1];
            let first_neighbors = graph.get(first).unwrap();
            let second_neighbors = graph.get(second).unwrap();
            let intersection: Vec<_> = first_neighbors
                .intersection(second_neighbors)
                .map(|x| *x)
                .collect();

            if intersection.is_empty() {
                return None;
            }

            if is_clique(&graph, &intersection) {
                let mut combination = vec![first, second];
                combination.extend(intersection);
                if !combination.iter().any(|x| x.starts_with("t")) {
                    return None;
                }
                Some(combination)
            } else {
                None
            }
        })
        .max_by_key(|x| x.len())
        .unwrap();

    let mut sorted_vertices = combination_vertices.clone();
    sorted_vertices.sort_unstable();

    let password = sorted_vertices.join(",");

    Ok(password.to_string())
}

fn build_graph(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut graph = HashMap::<&str, HashSet<&str>>::new();
    for line in input.lines() {
        let (left, right) = line.split_once("-").unwrap();
        graph.entry(left).or_insert(HashSet::new()).insert(right);
        graph.entry(right).or_insert(HashSet::new()).insert(left);
    }
    graph
}

fn is_clique(graph: &HashMap<&str, HashSet<&str>>, nodes: &Vec<&str>) -> bool {
    for i in 0..(nodes.len() - 1) {
        for j in (i + 1)..nodes.len() {
            let first = &nodes[i];
            let second = &nodes[j];
            if !graph.get(first).unwrap().contains(second) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_is_clique() -> miette::Result<()> {
        let graph = build_graph(include_str!("../input-example.txt"));
        assert!(is_clique(&graph, &vec!["aq", "cg", "yn"]));
        Ok(())
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "ka-co
ta-co
de-co
ta-ka
de-ta
ka-de";
        let result = process(input);
        assert_eq!("co,de,ka,ta", result?);
        Ok(())
    }
}
