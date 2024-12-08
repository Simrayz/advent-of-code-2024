use std::collections::{HashMap, HashSet};

pub mod part1;
pub mod part2;

pub struct Map {
    map: Vec<char>,
    antennas: HashMap<char, Vec<(i8, i8)>>,
    width: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let lines = input.lines();
        let mut width = 0;

        let (antennas, map) = lines
            .inspect(|x| {
                if width == 0 {
                    width = x.len();
                }
            })
            .enumerate()
            .fold(
                (HashMap::<char, Vec<(i8, i8)>>::new(), Vec::new()),
                |(mut map, mut acc), (y, line)| {
                    for (x, c) in line.chars().enumerate() {
                        if c != '.' {
                            map.entry(c).or_insert(Vec::new()).push((y as i8, x as i8));
                        }
                        acc.push(c);
                    }
                    (map, acc)
                },
            );

        Self {
            map,
            antennas,
            width,
        }
    }

    fn get_distance(&self, first: (i8, i8), second: (i8, i8)) -> (i8, i8) {
        ((second.0 - first.0), (second.1 - first.1))
    }

    fn within_bounds(&self, tuple: (i8, i8)) -> bool {
        tuple.0 >= 0 && tuple.0 < self.width as i8 && tuple.1 >= 0 && tuple.1 < self.width as i8
    }

    fn display_with_antinodes(&self, antinodes: &HashSet<(i8, i8)>) {
        for y in 0..self.width {
            for x in 0..self.width {
                if antinodes.contains(&(y as i8, x as i8)) && self.get(y, x) == '.' {
                    print!("#");
                } else {
                    print!("{}", self.get(y, x));
                }
            }
            println!();
        }
    }

    fn get(&self, y: usize, x: usize) -> char {
        self.map[self.get_flat_coord((y, x))]
    }

    fn get_flat_coord(&self, coord: (usize, usize)) -> usize {
        coord.0 * self.width as usize + coord.1
    }
}

pub fn subtract(a: (i8, i8), b: (i8, i8)) -> (i8, i8) {
    (a.0 - b.0, a.1 - b.1)
}

pub fn add(a: (i8, i8), b: (i8, i8)) -> (i8, i8) {
    (a.0 + b.0, a.1 + b.1)
}
