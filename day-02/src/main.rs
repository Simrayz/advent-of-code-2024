fn main() {
    let file = include_str!("../input2.txt");
    let result_part1 = process_part1(file);
    println!("Part 1: {}", result_part1);

    let result_part2 = process_part2(file);
    println!("Part 2: {}", result_part2);
}

fn process_part1(input: &str) -> usize {
    let rows = parse_rows(input);

    let safe_reports = rows.into_iter().filter(|row| valid_row(&row)).count();

    return safe_reports;
}

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

fn valid_row(row: &Vec<i64>) -> bool {
    let all_ascending = row.windows(2).all(|pair| pair[0] < pair[1]);
    let all_descending = row.windows(2).all(|pair| pair[0] > pair[1]);

    if !all_ascending && !all_descending {
        return false;
    }

    let valid_levels = row.windows(2).all(|pair| {
        let diff = (pair[1] - pair[0]).abs();
        return 1 <= diff && diff <= 3;
    });

    return valid_levels;
}

fn process_part2(input: &str) -> i32 {
    let rows = parse_rows(input);

    let safe_reports = rows
        .into_iter()
        .filter(|row| {
            if valid_row(row) {
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
                    valid_row(&new_row)
                };
                return is_valid;
            })
        })
        .count();

    dbg!(safe_reports);

    return safe_reports as i32;
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
