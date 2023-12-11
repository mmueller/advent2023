use crate::advent::AdventSolver;
use crate::util::io;
use anyhow::Error;

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error> {
        let lines = io::read_file_as_lines(input_path)?;
        let times = io::space_separated_numbers(&lines[0][10..])?;
        let distances = io::space_separated_numbers(&lines[1][10..])?;
        println!("Ways to win: {}", ways_to_beat_records(&times, &distances));

        // "Bad kerning" version
        let lines: Vec<String> = lines
            .iter()
            .map(|line| line.chars().filter(|c| c.is_ascii_digit()).collect())
            .collect();
        let time = lines[0].parse::<u64>()?;
        let distance = lines[1].parse::<u64>()?;
        println!("Ways to win: {}", ways_to_beat_record(time, distance));
        Ok(())
    }
}

fn hold_button_and_go(hold_time: u64, race_time: u64) -> u64 {
    hold_time * (race_time - hold_time)
}

fn ways_to_beat_record(time: u64, distance_record: u64) -> u64 {
    (1..time)
        .map(|h| hold_button_and_go(h, time))
        .filter(|&d| d > distance_record)
        .count() as u64
}

fn ways_to_beat_records(times: &Vec<u64>, distances: &Vec<u64>) -> u64 {
    times
        .iter()
        .zip(distances.iter())
        .fold(1, |ways, (&time, &distance)| {
            ways * ways_to_beat_record(time, distance)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(
            288,
            ways_to_beat_records(&vec![7, 15, 30], &vec![9, 40, 200])
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(71503, ways_to_beat_record(71530, 940200));
    }
}
