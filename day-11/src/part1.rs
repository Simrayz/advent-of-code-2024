pub fn process(start_input: &str) -> miette::Result<String> {
    let mut blinks = 0;
    let mut current_input = parse(start_input);

    while blinks < 25 {
        current_input = blink_once(&current_input);
        blinks += 1;
    }

    Ok(current_input.len().to_string())
}

fn parse(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn blink_once(input: &Vec<usize>) -> Vec<usize> {
    let mut new_input = Vec::new();
    input.into_iter().for_each(|x| {
        if *x == 0 {
            new_input.push(1);
            return;
        }
        let char_num = x.to_string();
        if char_num.len() % 2 == 0 {
            let half_idx = char_num.len() / 2;
            let (left, right) = char_num.split_at(half_idx);
            [left, right]
                .iter()
                .for_each(|x| new_input.push(x.parse().unwrap()))
        } else {
            new_input.push(x * 2024);
        }
    });

    return new_input;
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
        assert_eq!("", result?);
        Ok(())
    }
}
