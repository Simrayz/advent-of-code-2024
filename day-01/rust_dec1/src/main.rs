use std::{collections::HashMap, env, fs::read_to_string};

const FILE_PATH: &str = "../";

fn main() {
    let args: Vec<String> = env::args().collect();

    if !args.len() == 2 {
        panic!("Please provide the file name as the first argument");
    }
    let filename_from_path = &args[1];
    let filename = format!("{}{}", FILE_PATH, filename_from_path);

    let lines = read_lines(format!("{filename}").as_str());
    let (left_numbers, right_numbers) = get_location_id_lists(&lines);

    let distance = get_distance_score(&left_numbers, &right_numbers);
    let similarity = get_similarity_score(&left_numbers, &right_numbers);
    println!("Distance  : {:?}", distance);
    println!("Similarity: {:?}", similarity);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn get_location_id_lists(lines: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut left_numbers = Vec::<i32>::new();
    let mut right_numbers = Vec::<i32>::new();
    lines.iter().for_each(|line| {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        left_numbers.push(numbers[0]);
        right_numbers.push(numbers[1]);
    });

    left_numbers.sort();
    right_numbers.sort();

    return (left_numbers, right_numbers);
}

fn get_distance_score(left_numbers: &Vec<i32>, right_numbers: &Vec<i32>) -> i32 {
    let mut score = 0;
    for i in 0..left_numbers.len() {
        score += (left_numbers[i] - right_numbers[i]).abs();
    }
    return score;
}

fn get_similarity_score(left_numbers: &Vec<i32>, right_numbers: &Vec<i32>) -> i32 {
    let mut score = 0;
    let frequency_map = get_frequency_map(right_numbers);

    for value in left_numbers.iter() {
        if frequency_map.contains_key(value) {
            score += value * frequency_map.get(value).unwrap();
        }
    }

    return score;
}

fn get_frequency_map(numbers: &Vec<i32>) -> HashMap<i32, i32> {
    let mut map = HashMap::<i32, i32>::new();

    for number in numbers {
        *map.entry(*number).or_insert(0) += 1
    }

    return map;
}
