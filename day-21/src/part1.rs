use std::collections::HashMap;

fn create_direction_map() -> HashMap<char, HashMap<char, &str>> {
    let mut map = HashMap::new();
    map.insert(
        '^',
        [('>', "v>"), ('v', "v"), ('<', "v<")]
            .iter()
            .cloned()
            .collect(),
    );
    map.insert(
        'v',
        [('>', ">"), ('^', "^"), ('<', "<")]
            .iter()
            .cloned()
            .collect(),
    );
    map.insert(
        '<',
        [('>', ">>"), ('^', ">^"), ('v', ">")]
            .iter()
            .cloned()
            .collect(),
    );
    map.insert(
        '>',
        [('^', "<^"), ('v', "<"), ('<', "<<")]
            .iter()
            .cloned()
            .collect(),
    );
    map
}

pub fn process(input: &str) -> miette::Result<String> {
    let direction_map = create_direction_map();
    dbg!(&direction_map);
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input-example.txt"));
        assert_eq!("", result?);
        Ok(())
    }
}
