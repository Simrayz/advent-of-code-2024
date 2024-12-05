use std::collections::{HashMap, HashSet};

pub struct Solver {
    pub rules: HashMap<usize, HashSet<usize>>,
    pub updates: Vec<Vec<usize>>,
}

impl Solver {
    pub fn new(input: &str) -> Self {
        let mut rule_tuples = Vec::<(usize, usize)>::new();
        let mut updates = Vec::<Vec<usize>>::new();
        let mut rules_parsed = false;

        input.lines().into_iter().for_each(|line| {
            if line.is_empty() {
                rules_parsed = true;
                return;
            }
            if rules_parsed {
                // split string on , and parse to ints
                let numbers: Vec<usize> = line.split(",").map(|x| x.parse().unwrap()).collect();
                updates.push(numbers);
            } else {
                // split string on space
                let numbers: Vec<usize> = line.split("|").map(|x| x.parse().unwrap()).collect();
                rule_tuples.push((numbers[0], numbers[1]));
            }
        });

        let mut rules = HashMap::<usize, HashSet<usize>>::new();
        for (key, value) in rule_tuples.iter() {
            rules.entry(*key).or_insert(HashSet::new()).insert(*value);
        }

        Self { rules, updates }
    }

    pub fn validate_updates(&self) -> (Vec<&Vec<usize>>, Vec<&Vec<usize>>) {
        let (valid, invalid): (_, Vec<_>) = self
            .updates
            .iter()
            .partition(|update| self.validate_update(update));

        (valid, invalid)
    }

    pub fn validate_update(&self, update: &Vec<usize>) -> bool {
        update.into_iter().enumerate().all(|(i, value)| {
            let valid_after = &update[i + 1..]
                .into_iter()
                .all(|v| match self.rules.get(value) {
                    Some(rules) => rules.contains(v),
                    None => false,
                });
            return *valid_after;
        })
    }

    pub fn get_middle_page_sum(&self, updates: &Vec<&Vec<usize>>) -> usize {
        updates
            .iter()
            .map(|update| {
                let middle_index = update.len() / 2;
                update[middle_index]
            })
            .sum()
    }
}
