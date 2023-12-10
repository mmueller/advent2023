use anyhow::{format_err, Error};

trait AdventSolver {
    fn solve(&mut self, input_path: &str) -> Result<(), anyhow::Error>;
}

pub mod day01;
pub mod day02;

pub fn solve(day: u32) -> Result<(), Error> {
    let mut solver: Box<dyn AdventSolver> = match day {
        1 => Box::new(day01::Solver::default()),
        2 => Box::new(day02::Solver::default()),
        _ => {
            return Err(format_err!("Day {} not implemented.", day));
        }
    };
    solver.solve(&format!("inputs/day{:02}.txt", day))
}
