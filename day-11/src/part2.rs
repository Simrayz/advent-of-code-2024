use num_traits::Euclid;
use std::collections::HashMap;

pub fn process(start_input: &str) -> miette::Result<String> {
    let start_input = parse(start_input);
    let final_input = process_blinks(&start_input, 75);

    Ok(final_input.to_string())
}

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn process_blinks(input: &Vec<u64>, blinks: u64) -> u64 {
    let mut cache: HashMap<u64, u64> = HashMap::new();

    for num in input {
        cache.entry(*num).and_modify(|v| *v += 1).or_insert(1);
    }

    for _ in 0..blinks {
        let mut new_cache: HashMap<u64, u64> = HashMap::default();

        for (number, count) in cache.iter() {
            match number {
                0 => {
                    new_cache
                        .entry(1)
                        .and_modify(|v| *v += count)
                        .or_insert(*count);
                }
                n if (n.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 => {
                    let digits = n.checked_ilog10().unwrap_or(0) + 1;
                    let (left, right) = n.div_rem_euclid(&10u64.pow(digits / 2));
                    new_cache
                        .entry(left)
                        .and_modify(|v| {
                            *v += count;
                        })
                        .or_insert(*count);
                    new_cache
                        .entry(right)
                        .and_modify(|v| {
                            *v += count;
                        })
                        .or_insert(*count);
                }
                n => {
                    new_cache
                        .entry(n * 2024)
                        .and_modify(|v| {
                            *v += count;
                        })
                        .or_insert(*count);
                }
            }
        }
        cache = new_cache;
    }

    return cache.values().sum::<u64>();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input1.txt");

    #[test_log::test]
    fn test_parse() {
        assert_eq!(vec![125, 17], parse(INPUT));
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(INPUT);
        assert_eq!("65601038650482", result?);
        Ok(())
    }
}
