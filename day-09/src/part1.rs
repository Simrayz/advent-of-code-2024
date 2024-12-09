use std::{collections::HashMap, iter};

pub fn process(input: &str) -> miette::Result<String> {
    let expanded = expand_input(input);

    let new_string = get_sorted_input(&expanded);
    let check_sum = get_check_sum(&new_string);

    Ok(check_sum.to_string())
}

const SPACE: i64 = -1;

fn expand_input(input: &str) -> Vec<i64> {
    let expanded_format = input
        .chars()
        .enumerate()
        .flat_map(|(idx, c)| {
            let amount = c.to_digit(10).unwrap();
            let is_even = idx % 2 == 0;

            let repeat_char: i64 = match is_even {
                true => idx as i64 / 2,
                false => SPACE,
            };

            iter::repeat(repeat_char).take(amount as usize)
        })
        .collect();

    expanded_format
}

fn get_sorted_input(input: &Vec<i64>) -> Vec<i64> {
    let (empty, numbers): (Vec<_>, Vec<_>) = input.iter().enumerate().partition(|(_, c)| **c == -1);

    let movement_plan: HashMap<_, _> = empty
        .iter()
        .zip(numbers.iter().rev())
        .map(|(a, b)| (a.0, b.0))
        .collect();

    input
        .iter()
        .enumerate()
        .map(|(idx, c)| {
            if idx > numbers.len() - 1 {
                return -1;
            }
            movement_plan.get(&idx).map_or(*c, |&x| input[x])
        })
        .collect()

    // let (empty, numbers) = input.iter().enumerate().fold(
    //     (Vec::new(), Vec::new()),
    //     |(mut empty, mut acc), (idx, c)| {
    //         if *c == '.' {
    //             empty.push(idx);
    //             (empty, acc)
    //         } else {
    //             acc.push(idx);
    //             (empty, acc)
    //         }
    //     },
    // );
    // let movement: HashMap<_, _> = empty
    //     .iter()
    //     .zip(numbers.iter().rev())
    //     .map(|(a, b)| (*a, *b))
    //     .collect();

    // let number_length = numbers.len();
    // input
    //     .iter()
    //     .enumerate()
    //     .map(|(idx, c)| {
    //         if idx > number_length - 1 {
    //             return '.';
    //         }
    //         match movement.get(&idx) {
    //             Some(x) => {
    //                 return input[*x];
    //             }
    //             None => *c,
    //         }
    //     })
    //     .collect()
}

fn get_check_sum(input: &Vec<i64>) -> i64 {
    let mut sum = 0;
    for (i, char) in input.iter().enumerate() {
        if *char == -1 {
            break;
        }
        sum += *char * (i as i64);
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_expand_input() {
        let input = "23451";
        let answer = [0, 0, -1, -1, -1, 1, 1, 1, 1, -1, -1, -1, -1, -1, 2].to_vec();
        assert_eq!(answer, expand_input(input));
    }

    #[test_log::test]
    fn test_get_sorted_input() {
        let input = [0, 0, -1, -1, -1, 1, 1, 1, 1, -1, -1, -1, -1, -1, 2].to_vec();
        let answer: Vec<i64> = "0021111........"
            .chars()
            .map(|c| match c.to_digit(10) {
                Some(x) => x as i64,
                None => -1,
            })
            .collect();
        assert_eq!(answer, get_sorted_input(&input));
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input1.txt"));
        assert_eq!("1928", result?);
        Ok(())
    }
}
