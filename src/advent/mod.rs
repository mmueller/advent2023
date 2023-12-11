use anyhow::{format_err, Error};

trait AdventSolver {
    fn solve(&mut self, input_path: &str) -> Result<(), anyhow::Error>;
}

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

pub fn solve(day: u32) -> Result<(), Error> {
    let mut solver: Box<dyn AdventSolver> = match day {
        1 => Box::new(day01::Solver::default()),
        2 => Box::new(day02::Solver::default()),
        3 => Box::new(day03::Solver::default()),
        4 => Box::new(day04::Solver::default()),
        5 => Box::new(day05::Solver::default()),
        6 => Box::new(day06::Solver::default()),
        _ => {
            return Err(format_err!("Day {} not implemented.", day));
        }
    };
    solver.solve(&format!("inputs/day{:02}.txt", day))
}
