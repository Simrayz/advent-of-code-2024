use std::collections::HashSet;

pub fn process(input: &str) -> miette::Result<String> {
    let mut connections = HashSet::<(&str, &str)>::new();
    let mut computer_ids = HashSet::<&str>::new();
    for line in input.lines() {
        let (left, right) = line.split_once("-").unwrap();
        connections.insert((left, right));
        connections.insert((right, left));
        computer_ids.insert(left);
    }

    let mut unique_sets = HashSet::<Vec<&str>>::new();
    for (a, b) in connections.iter() {
        if !a.starts_with("t") {
            continue;
        }
        for computer in computer_ids.iter() {
            if connections.contains(&(a, computer)) && connections.contains(&(b, computer)) {
                let mut set = vec![*a, *b, *computer];
                set.sort_unstable();
                unique_sets.insert(set);
            }
        }
    }

    Ok(unique_sets.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input-example.txt"));
        assert_eq!("7", result?);
        Ok(())
    }
}
