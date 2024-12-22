use rayon::prelude::*;
use std::{collections::HashMap, iter::once};

use crate::parse_input;

pub fn process(input: &str) -> miette::Result<String> {
    let numbers = parse_input::<isize>(input).unwrap();

    let counts = numbers
        .par_iter()
        .flat_map(|&secret| {
            let prices = get_prices(secret, 2000);
            let diffs: Vec<_> = prices.windows(2).map(|diff| diff[1] - diff[0]).collect();

            diffs
                .windows(4)
                .enumerate()
                .fold(HashMap::new(), |mut acc, (idx, diff)| {
                    let key = diff.to_vec();
                    if !acc.contains_key(&key) {
                        acc.insert(key, prices[idx + 4]);
                    };
                    acc
                })
        })
        .fold(
            || HashMap::new(),
            |mut acc, (key, value)| {
                *acc.entry(key).or_insert(0) += value;
                acc
            },
        )
        .reduce(
            || HashMap::new(),
            |mut a, b| {
                for (key, value) in b {
                    *a.entry(key).or_insert(0) += value;
                }
                a
            },
        );

    let result = counts.values().max().ok_or("no max value found").unwrap();

    Ok(result.to_string())
}

pub fn get_prices(secret: isize, time: usize) -> Vec<isize> {
    once(secret % 10)
        .chain((0..time).scan(secret, |state, _| {
            let next = calculate_next(*state);
            *state = next;
            Some(next % 10)
        }))
        .collect::<Vec<isize>>()
}

pub fn calculate_n_secrets(secret: isize, time: usize) -> (Vec<isize>, Vec<isize>) {
    let mut prices = Vec::<isize>::new();
    let mut diffs = Vec::<isize>::new();
    (0..time).fold(secret, |acc, _| {
        let next = calculate_next(acc);
        prices.push(next % 10);
        diffs.push(next % 10 - acc % 10);
        next
    });
    (prices, diffs)
}

pub fn calculate_next(mut secret: isize) -> isize {
    secret = ((secret * 64) ^ secret) % 16_777_216;
    secret = ((secret / 32) ^ secret) % 16_777_216;
    ((secret * 2048) ^ secret) % 16_777_216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input-example.txt"));
        assert_eq!("24", result?);
        Ok(())
    }
}
