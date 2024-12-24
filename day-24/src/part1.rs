use std::collections::{HashMap, HashSet};

use crate::{parse_file, Gate, GateType};

pub fn process(input: &str) -> miette::Result<String> {
    let (inputs, gates) = parse_file(input);

    let mut seen_inputs: HashMap<&String, usize> =
        HashMap::from_iter(inputs[0..2].iter().map(|i| (&i.0, i.1)));
    let mut used_gates: HashSet<&Gate> = HashSet::new();
    let mut current_index = 2;

    while used_gates.len() < gates.len() {
        for gate in gates.iter() {
            if used_gates.contains(gate) {
                continue;
            }
            if seen_inputs.contains_key(&gate.left) && seen_inputs.contains_key(&gate.right) {
                let left = seen_inputs.get(&gate.left).unwrap();
                let right = seen_inputs.get(&gate.right).unwrap();
                let result = match gate.gate {
                    GateType::AND => left & right,
                    GateType::OR => left | right,
                    GateType::XOR => left ^ right,
                };
                // println!(
                //     "({}, {}) = {} {:?} {}",
                //     gate.output, result, left, gate.gate, right
                // );

                seen_inputs.insert(&gate.output, result);
                used_gates.insert(&gate);
            }
        }
        if current_index < inputs.len() {
            let next_input = &inputs[current_index];
            seen_inputs.insert(&next_input.0, next_input.1);
            current_index += 1;
        }
    }

    let mut z_inputs: Vec<_> = seen_inputs
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .collect();
    z_inputs.sort_unstable_by(|(a, _), (b, _)| b.cmp(a));

    let output = z_inputs
        .iter()
        .map(|(_, v)| v.to_string())
        .collect::<String>();

    let result = isize::from_str_radix(output.as_str(), 2).expect("A valid binary string");

    // println!("{:?} = {}", output, result);

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input-example.txt"));
        assert_eq!("2024", result?);
        Ok(())
    }
}
