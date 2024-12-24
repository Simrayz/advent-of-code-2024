use std::collections::VecDeque;

use itertools::Itertools;

pub mod part1;
pub mod part2;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum GateType {
    XOR,
    OR,
    AND,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Gate {
    left: String,
    right: String,
    output: String,
    gate: GateType,
}

pub fn parse_file(input: &str) -> (Vec<(String, usize)>, VecDeque<Gate>) {
    let (input_lines, gate_lines) = input.split_once("\n\n").unwrap();

    let inputs = input_lines
        .lines()
        .map(|l| {
            let (name, value) = l.split(": ").collect_tuple().unwrap();
            (name.to_string(), value.parse::<usize>().unwrap())
        })
        .collect::<Vec<(String, usize)>>();

    let gates: VecDeque<_> = gate_lines
        .lines()
        .map(|l| {
            let pieces: Vec<_> = l.split_whitespace().collect();

            Gate {
                left: pieces[0].to_string(),
                right: pieces[2].to_string(),
                output: pieces[4].to_string(),
                gate: get_gate_type(pieces[1]),
            }
        })
        .collect();

    (inputs, gates)
}

fn get_gate_type(input: &str) -> GateType {
    match input {
        "AND" => GateType::AND,
        "OR" => GateType::OR,
        "XOR" => GateType::XOR,
        _ => unreachable!(),
    }
}
