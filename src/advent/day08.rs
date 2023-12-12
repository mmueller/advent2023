use crate::advent::AdventSolver;
use crate::util::io;
use anyhow::{format_err, Error};
use lazy_static::lazy_static;
use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use strum::{self, EnumString};

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error> {
        let input = io::read_file_as_lines(input_path)?;
        let map = DesertMap::new(&input)?;
        println!("Steps to ZZZ: {}", map.steps_to("ZZZ"));
        println!("Steps to ??Z: {}", map.parallel_steps_to_z());
        Ok(())
    }
}

lazy_static! {
    static ref MAP_ELEMENT: Regex =
        Regex::new(r"^(?P<start>\w+) = \((?P<left>\w+), (?P<right>\w+)\)$").unwrap();
}

#[derive(Clone, Copy, Debug, EnumString)]
enum Step {
    #[strum(serialize = "L")]
    Left,
    #[strum(serialize = "R")]
    Right,
}

struct DesertMap {
    map: HashMap<String, (String, String)>,
    steps: Vec<Step>,
}

impl DesertMap {
    fn new<S: AsRef<str>>(input: &Vec<S>) -> Result<Self, Error> {
        let steps = input[0]
            .as_ref()
            .chars()
            .map(|c| Step::from_str(&c.to_string()))
            .collect::<Result<Vec<_>, _>>()?;

        let mut map = HashMap::new();
        for i in 2..input.len() {
            let line = input[i].as_ref();
            let caps = MAP_ELEMENT
                .captures(line)
                .ok_or(format_err!("Couldn't parse element: {}", line))?;
            map.insert(
                caps["start"].to_string(),
                (caps["left"].to_string(), caps["right"].to_string()),
            );
        }

        Ok(DesertMap { map, steps })
    }

    fn steps_to(&self, target: &str) -> u64 {
        let mut total_steps = 0;
        let mut current_node = "AAA";
        let mut current_step = 0;
        while current_node != target {
            let next_nodes = &self.map[current_node];
            current_node = match self.steps[current_step] {
                Step::Left => &next_nodes.0,
                Step::Right => &next_nodes.1,
            };
            current_step = (current_step + 1) % self.steps.len();
            total_steps += 1;
        }
        total_steps
    }

    fn parallel_steps_to_z(&self) -> u64 {
        // Each ending node ??Z will be on a cycle of length L. If we start ??A somewhere in the
        // middle of that cycle, then there would be some offset in addition to a multiple of
        // cycles to consider when trying to land on ??Z. (Similarly, if we step from ??A into some
        // cycle that doesn't include ??A, it may have a different length than the offset.) Then we
        // would have to figure out the offset and length for each cycle and then find a number of
        // steps that lines all the cycles up on the ending nodes ??Z.
        //
        // However, experimentally it seems my input has the cycle length == initial offset, so the
        // answer is a simple LCM of the cycle lengths.
        let mut cycle_lengths: Vec<u64> = Vec::new();
        for starting_node in self.map.keys().filter(|k| k.ends_with("A")) {
            let mut current_step = 0;
            let mut steps = 0;
            let mut current_node = starting_node;
            loop {
                let next_nodes = &self.map[current_node];
                current_node = match self.steps[current_step] {
                    Step::Left => &next_nodes.0,
                    Step::Right => &next_nodes.1,
                };
                current_step = (current_step + 1) % self.steps.len();
                steps += 1;
                if current_node.ends_with("Z") {
                    cycle_lengths.push(steps);
                    break;
                }
            }
        }

        cycle_lengths
            .iter()
            .fold(1, |result, &cycle| lcm(result, cycle))
    }
}

#[cfg(test)]
mod tests {
    use super::DesertMap;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref EX1_IN: Vec<&'static str> = vec![
            "RL",
            "",
            "AAA = (BBB, CCC)",
            "BBB = (DDD, EEE)",
            "CCC = (ZZZ, GGG)",
            "DDD = (DDD, DDD)",
            "EEE = (EEE, EEE)",
            "GGG = (GGG, GGG)",
            "ZZZ = (ZZZ, ZZZ)",
        ];
        static ref EX2_IN: Vec<&'static str> = vec![
            "LLR",
            "",
            "AAA = (BBB, BBB)",
            "BBB = (AAA, ZZZ)",
            "ZZZ = (ZZZ, ZZZ)",
        ];
        static ref EX3_IN: Vec<&'static str> = vec![
            "LR",
            "",
            "11A = (11B, XXX)",
            "11B = (XXX, 11Z)",
            "11Z = (11B, XXX)",
            "22A = (22B, XXX)",
            "22B = (22C, 22C)",
            "22C = (22Z, 22Z)",
            "22Z = (22B, 22B)",
            "XXX = (XXX, XXX)",
        ];
    }

    #[test]
    fn test_ex1_steps_to_zzz() {
        let map = DesertMap::new(&EX1_IN).unwrap();
        assert_eq!(2, map.steps_to("ZZZ"));
    }

    #[test]
    fn test_ex2_steps_to_zzz() {
        let map = DesertMap::new(&EX2_IN).unwrap();
        assert_eq!(6, map.steps_to("ZZZ"));
    }

    #[test]
    fn test_ex3_parallel_steps_to_z() {
        let map = DesertMap::new(&EX3_IN).unwrap();
        assert_eq!(6, map.parallel_steps_to_z());
    }
}
