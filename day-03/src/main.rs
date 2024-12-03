fn main() {
    let file = include_str!("../input1.txt");
    println!("Part 1: {}", process_part1(file));
    println!("Part 2: {}", process_part2(file));
}

// Part 1
fn process_part1(input: &str) -> i64 {
    let patterns = get_multiplication_patterns(input);
    return patterns.into_iter().map(get_multiplication_result).sum();
}

fn get_multiplication_patterns(input: &str) -> Vec<String> {
    // regex that gets all mul(number, number)
    let re = regex::Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let results: Vec<String> = re
        .find_iter(input)
        .filter_map(|mul| mul.as_str().parse().ok())
        .collect();
    return results;
}

fn get_multiplication_result(multiplication_pattern: String) -> i64 {
    let re = regex::Regex::new(r"[0-9]+").unwrap();
    let result: i64 = re
        .find_iter(&multiplication_pattern)
        .filter_map(|x| x.as_str().parse::<i64>().ok())
        .reduce(|a, b| a * b)
        .unwrap();
    return result;
}

// Part 2
fn process_part2(input: &str) -> i64 {
    let instructions = process_valid_instructions(input);
    return instructions
        .into_iter()
        .map(get_multiplication_result)
        .sum();
}
fn process_valid_instructions(input: &str) -> Vec<String> {
    let instructions = get_valid_instructions(input);

    let mut filtered_instructions: Vec<String> = Vec::new();

    let mut include_instructions = true;
    for instruction in instructions {
        match instruction.as_str() {
            "do()" => include_instructions = true,
            "don't()" => include_instructions = false,
            _ => {
                if include_instructions {
                    filtered_instructions.push(instruction);
                }
            }
        }
    }

    return filtered_instructions;
}

fn get_valid_instructions(input: &str) -> Vec<String> {
    let re = regex::Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)").unwrap();
    let results: Vec<String> = re
        .find_iter(input)
        .filter_map(|mul| mul.as_str().parse().ok())
        .collect();
    return results;
}

#[cfg(test)]

mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_get_multiplication_patterns() {
        let left_side: Vec<String> = ["mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)"]
            .into_iter()
            .map(String::from)
            .collect();

        assert_eq!(left_side, get_multiplication_patterns(TEST_INPUT));
    }

    #[test]
    fn test_process_part1() {
        assert_eq!(161, process_part1(TEST_INPUT));
    }

    #[test]
    fn test_get_valid_instructions() {
        let left_side: Vec<String> = [
            "mul(2,4)",
            "don't()",
            "mul(5,5)",
            "mul(11,8)",
            "do()",
            "mul(8,5)",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        assert_eq!(left_side, get_valid_instructions(TEST_INPUT));
    }

    #[test]
    fn test_process_valid_instructions() {
        assert_eq!(
            Vec::from(["mul(2,4)", "mul(8,5)"].map(String::from)),
            process_valid_instructions(TEST_INPUT)
        );
    }

    #[test]
    fn test_process_part2() {
        assert_eq!(48, process_part2(TEST_INPUT));
    }
}
