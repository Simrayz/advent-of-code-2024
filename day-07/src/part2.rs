use itertools::{repeat_n, Itertools};

pub fn process(input: &str) -> miette::Result<String> {
    let lines: Vec<(usize, Vec<usize>)> = input
        .lines()
        .map(|line| {
            let components: Vec<_> = line
                .replace(":", "")
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>();

            let head = components[0];
            let tail = components[1..].to_vec();

            (head, tail)
        })
        .collect();

    println!("{:?}", lines);

    // let max_line = lines
    //     .iter()
    //     .map(|x| &x.1)
    //     .minmax_by(|a, b| a.len().cmp(&b.len()));

    let results = lines
        .iter()
        .filter_map(|line| match process_line(line) {
            true => Some(line.0),
            false => None,
        })
        .collect::<Vec<usize>>();
    let sum = results.iter().fold(0, |acc, x| acc + x);

    println!("results: {:?}, sum: {}", results, sum);

    Ok(sum.to_string())
}

fn process_line(line: &(usize, Vec<usize>)) -> bool {
    let operators = vec![Operator::ADD, Operator::MULTIPLY, Operator::CONCATENATE];
    let permutations = get_permutations(&operators, line.1.len() - 1);

    let mut result_found = false;

    for operations in permutations {
        let mut result = line.1[0];
        for (i, op) in operations.iter().enumerate() {
            let b = line.1[i + 1];
            result = op.execute(result, b);
        }
        if result == line.0 {
            result_found = true;
            break;
        }
    }
    return result_found;
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    ADD,
    MULTIPLY,
    CONCATENATE,
}

impl Operator {
    fn execute(self, a: usize, b: usize) -> usize {
        match self {
            Operator::ADD => a + b,
            Operator::MULTIPLY => a * b,
            Operator::CONCATENATE => {
                let result = format!("{}{}", a.to_string(), b.to_string());
                result.parse().unwrap()
            }
        }
    }
}

fn get_permutations(operators: &Vec<Operator>, n: usize) -> Vec<Vec<Operator>> {
    let operations: Vec<Vec<Operator>> = repeat_n(operators.clone(), n)
        .multi_cartesian_product()
        .collect();
    return operations;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input1.txt"));
        assert_eq!("3749", result?);
        Ok(())
    }
}
