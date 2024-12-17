pub mod part1;
pub mod part2;

#[derive(Debug, Eq, PartialEq)]
pub struct Computer {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub program: Vec<usize>,
    pub pointer: usize,
    pub outputs: Vec<usize>,
}

impl Default for Computer {
    fn default() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            program: Vec::new(),
            pointer: 0,
            outputs: Vec::new(),
        }
    }
}

impl Computer {
    pub fn run(&mut self) -> Vec<usize> {
        while self.pointer < self.program.len() {
            self.perform_operation();
        }
        self.outputs.clone()
    }
    pub fn copy_with_a(&self, a: usize) -> Self {
        Self {
            a,
            b: self.b,
            c: self.c,
            program: self.program.clone(),
            pointer: self.pointer,
            outputs: Vec::new(),
        }
    }
    pub fn to_output(&self) -> String {
        // join output numbers into String
        self.outputs
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
    pub fn perform_operation(&mut self) {
        let operation = self.program[self.pointer];
        let operand = self.program[self.pointer + 1];

        if operation == 3 && self.a != 0 {
            self.pointer = operand as usize;
            return;
        }

        match operation {
            0 => {
                let numerator = self.a;
                let operand = self.get_combo_operand(operand);
                let denominator = usize::pow(2, operand as u32);
                self.a = numerator / denominator;
            }
            1 => {
                self.b = self.b ^ operand as usize;
            }
            2 => {
                let operand = self.get_combo_operand(operand);
                self.b = operand % 8;
            }
            3 => {}
            4 => {
                self.b = self.b ^ self.c;
            }
            5 => {
                let operand = self.get_combo_operand(operand);
                self.outputs.push(operand % 8);
            }
            6 => {
                let numerator = self.a;
                let operand = self.get_combo_operand(operand);
                let denominator = usize::pow(2, operand as u32);
                self.b = numerator / denominator;
            }
            7 => {
                let numerator = self.a;
                let operand = self.get_combo_operand(operand);
                let denominator = usize::pow(2, operand as u32);
                self.c = numerator / denominator;
            }
            _ => panic!("Unknown operation"),
        }

        self.pointer += 2;
    }
    pub fn get_combo_operand(&self, operand: usize) -> usize {
        match operand {
            num if num <= 3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Unknown operand"),
        }
    }
    pub fn from_input(input: &str) -> Self {
        let mut registers: Vec<usize> = Vec::new();
        let mut instructions: Vec<usize> = Vec::new();

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }
            if line.starts_with("Register") {
                let value = line.split_whitespace().last().unwrap().parse().unwrap();
                registers.push(value);
            }
            if line.starts_with("Program") {
                instructions = line
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .split(",")
                    .map(|x| x.parse().unwrap())
                    .collect();
            }
        }

        Self {
            a: registers[0],
            b: registers[1],
            c: registers[2],
            program: instructions,
            pointer: 0,
            outputs: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = Computer::from_input(include_str!("../input-example.txt"));
        let test_register = Computer {
            a: 729,
            b: 0,
            c: 0,
            program: vec![0, 1, 5, 4, 3, 0],
            ..Default::default()
        };
        assert_eq!(test_register, result);
        Ok(())
    }
}
