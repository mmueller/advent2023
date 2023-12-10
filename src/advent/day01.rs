use crate::advent::AdventSolver;
use crate::util::conversions::digit_value;
use crate::util::io;
use anyhow::Error;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Default)]
pub struct Solver;

lazy_static! {
    static ref DIGIT_REGEX: Regex =
        Regex::new(r"^(one|two|three|four|five|six|seven|eight|nine)").unwrap();
}

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error> {
        let input = io::read_file_as_lines(input_path)?;

        // Part 1: ASCII digits only
        let calibration_values1 = input
            .iter()
            .map(|line| get_calibration_value(&line, false))
            .collect::<Result<Vec<u64>, _>>()?;
        println!(
            "Sum of calibration values: {}",
            calibration_values1.iter().sum::<u64>()
        );

        // Part 2: Include spelled-out numbers
        let calibration_values2 = input
            .iter()
            .map(|line| get_calibration_value(&line, true))
            .collect::<Result<Vec<u64>, _>>()?;
        println!(
            "Fixed sum of calibration values: {}",
            calibration_values2.iter().sum::<u64>()
        );
        Ok(())
    }
}

pub fn get_calibration_value(s: &str, include_spelled_out_numbers: bool) -> Result<u64, Error> {
    let mut digit1 = 0;
    let mut digit2 = 0;

    for i in 0..s.len() {
        if let Some(next_digit) = get_digit(&s[i..], include_spelled_out_numbers) {
            if digit1 == 0 {
                digit1 = next_digit;
            }
            digit2 = next_digit;
        }
    }
    Ok(digit1 * 10 + digit2)
}

// What an annoying day 1
fn get_digit(s: &str, include_spelled_out_numbers: bool) -> Option<u64> {
    if let Some(c) = s.chars().nth(0) {
        if c.is_ascii_digit() {
            Some(digit_value(c))
        } else if include_spelled_out_numbers {
            DIGIT_REGEX.find(s).map(|m| match m.as_str() {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => unreachable!(),
            })
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::get_calibration_value;

    const EX1_IN: &[&str] = &["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    const EX1_OUT: u64 = 142;

    const EX2_IN: &[&str] = &[
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ];
    const EX2_OUT: u64 = 281;

    #[test]
    fn test_calibration_value1() {
        assert_eq!(12, get_calibration_value(EX1_IN[0], false).unwrap());
        assert_eq!(38, get_calibration_value(EX1_IN[1], false).unwrap());
        assert_eq!(15, get_calibration_value(EX1_IN[2], false).unwrap());
        assert_eq!(77, get_calibration_value(EX1_IN[3], false).unwrap());
    }

    #[test]
    fn test_calibration_value_sum1() {
        let sum = EX1_IN
            .iter()
            .map(|s| get_calibration_value(s, false).unwrap())
            .sum();
        assert_eq!(EX1_OUT, sum);
    }

    #[test]
    fn test_calibration_value2() {
        assert_eq!(29, get_calibration_value(EX2_IN[0], true).unwrap());
        assert_eq!(83, get_calibration_value(EX2_IN[1], true).unwrap());
        assert_eq!(13, get_calibration_value(EX2_IN[2], true).unwrap());
        assert_eq!(24, get_calibration_value(EX2_IN[3], true).unwrap());
        assert_eq!(42, get_calibration_value(EX2_IN[4], true).unwrap());
        assert_eq!(14, get_calibration_value(EX2_IN[5], true).unwrap());
        assert_eq!(76, get_calibration_value(EX2_IN[6], true).unwrap());
    }

    #[test]
    fn test_calibration_value_sum2() {
        let sum = EX2_IN
            .iter()
            .map(|s| get_calibration_value(s, true).unwrap())
            .sum();
        assert_eq!(EX2_OUT, sum);
    }
}
