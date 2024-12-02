fn main() {
    let file = include_str!("../input2.txt");
    let result_part1 = process_part1(file);
    println!("Part 1: {}", result_part1);

    let result_part2 = process_part2(file);
    println!("Part 2: {}", result_part2);
}

// Part 1 - Find number of safe rows
fn process_part1(input: &str) -> usize {
    let rows = parse_rows(input);
    let safe_reports = rows.into_iter().filter(check_safety).count();
    return safe_reports;
}

// Common utilities
fn parse_rows(input: &str) -> Vec<Vec<i64>> {
    let mut rows = Vec::<Vec<i64>>::new();
    for line in input.lines() {
        let parts = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i64>>();
        rows.push(parts);
    }

    return rows;
}

fn check_safety(row: &Vec<i64>) -> bool {
    let safety_scores = (0..row.len() - 1)
        .map(|i| row[i] - row[i + 1])
        .collect::<Vec<i64>>();

    let valid_ascending = safety_scores.iter().all(|x| *x >= 1 && *x <= 3);
    let valid_descending = safety_scores.iter().all(|x| *x <= -1 && *x >= -3);

    return valid_ascending || valid_descending;
}
// End common utilities

// Part 2 - Find number of safe rows with Problem Dampener
fn process_part2(input: &str) -> i32 {
    let rows = parse_rows(input);
    let safe_reports = rows.into_iter().filter(check_safety_with_dampening).count();

    return safe_reports as i32;
}

fn check_safety_with_dampening(row: &Vec<i64>) -> bool {
    if check_safety(row) {
        return true;
    }
    row.iter().enumerate().any(|(i, _)| {
        let is_valid = {
            let new_row = row
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, &val)| val)
                .collect::<Vec<i64>>();
            check_safety(&new_row)
        };
        return is_valid;
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process_part1(input).to_string());
    }
    #[test]
    fn test_process_part2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", process_part2(input).to_string());
    }
}
