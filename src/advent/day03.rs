use crate::advent::AdventSolver;
use crate::util::conversions::digit_value;
use crate::util::io;
use anyhow::Error;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error> {
        let input = io::read_file_as_lines(input_path)?;
        let schematic = EngineSchematic::new(&input);
        println!(
            "Sum of part numbers: {}",
            schematic.get_part_numbers().iter().sum::<u64>()
        );
        println!(
            "Sum of gear ratios: {}",
            schematic.get_gear_ratios().iter().sum::<u64>()
        );
        Ok(())
    }
}

struct EngineSchematic {
    /// Raw data
    data: Vec<String>,

    /// Position of symbols as tuple (row, column)
    symbols: HashSet<(usize, usize)>,

    /// Position of numbers as tuple (row, column, length, value)
    numbers: Vec<(usize, usize, usize, u64)>,
}

impl EngineSchematic {
    pub fn new(lines: &Vec<String>) -> EngineSchematic {
        let mut symbols = HashSet::new();
        let mut numbers = Vec::new();
        let mut current_number: Option<(usize, usize, usize, u64)> = None;
        for (row, line) in lines.iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    if let Some(ref mut num) = current_number {
                        // Current number length grows by one, value updates accordingly
                        num.2 += 1;
                        num.3 = num.3 * 10 + digit_value(c);
                    } else {
                        current_number = Some((row, col, 1, digit_value(c)));
                    }
                } else {
                    if let Some(num) = current_number {
                        numbers.push(num);
                        current_number = None;
                    }
                    if c != '.' {
                        symbols.insert((row, col));
                    }
                }
            }
            if let Some(num) = current_number {
                numbers.push(num);
                current_number = None;
            }
        }

        EngineSchematic {
            data: lines.clone(),
            symbols: symbols,
            numbers: numbers,
        }
    }

    pub fn get_part_numbers(&self) -> Vec<u64> {
        self.numbers
            .iter()
            .filter(|n| {
                self.adjacent_positions(n)
                    .iter()
                    .any(|p| self.symbols.contains(p))
            })
            .map(|n| n.3)
            .collect()
    }

    pub fn get_gear_ratios(&self) -> Vec<u64> {
        // Lookup table of * symbol position -> adjacent numbers
        let mut adjacent_numbers: HashMap<(usize, usize), Vec<u64>> = HashMap::new();
        for n in self.numbers.iter() {
            for p in self.adjacent_positions(n).iter() {
                if self.data[p.0].chars().nth(p.1).unwrap() == '*' {
                    adjacent_numbers.entry(*p).or_default().push(n.3);
                }
            }
        }
        // Find all the * with exactly two numbers adjacent
        adjacent_numbers
            .iter()
            .filter(|(_p, n)| n.len() == 2)
            .map(|(_p, n)| n[0] * n[1])
            .collect()
    }

    fn adjacent_positions(&self, number_tuple: &(usize, usize, usize, u64)) -> Vec<(usize, usize)> {
        let (row, col, len, _value) = *number_tuple;
        let mut result = Vec::new();

        // Positions before the number
        if col > 0 {
            if row > 0 {
                result.push((row - 1, col - 1));
            }
            result.push((row, col - 1));
            if row + 1 < self.data.len() {
                result.push((row + 1, col - 1));
            }
        }

        // Positions above and below the number
        for c in col..col + len {
            if row > 0 {
                result.push((row - 1, c));
            }
            if row + 1 < self.data.len() {
                result.push((row + 1, c));
            }
        }

        // Positions after the number
        let col_after = col + len;
        if col_after < self.data[0].len() {
            if row > 0 {
                result.push((row - 1, col_after));
            }
            result.push((row, col_after));
            if row + 1 < self.data.len() {
                result.push((row + 1, col_after));
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::EngineSchematic;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref EX_IN: Vec<String> = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];
    }

    #[test]
    fn test_sum_of_part_numbers() {
        let part_numbers = EngineSchematic::new(&EX_IN).get_part_numbers();
        assert_eq!(4361u64, part_numbers.iter().sum());
    }

    #[test]
    fn test_sum_of_gear_ratios() {
        let gear_ratios = EngineSchematic::new(&EX_IN).get_gear_ratios();
        assert_eq!(467835u64, gear_ratios.iter().sum());
    }
}
