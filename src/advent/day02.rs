use crate::advent::AdventSolver;
use anyhow::{format_err, Error};
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default)]
pub struct Solver;

lazy_static! {
    static ref GAME_RE: Regex = Regex::new(r"Game (?P<id>\d+): (?P<results>.*)$").unwrap();
    static ref RESULT_RE: Regex = Regex::new(r"(?P<count>\d+) (?P<color>red|green|blue)").unwrap();
}

const MAX_RED: u64 = 12;
const MAX_GREEN: u64 = 13;
const MAX_BLUE: u64 = 14;

#[derive(Default)]
struct Game {
    id: u64,
    results: Vec<GameResult>,
}

#[derive(Default)]
struct GameResult {
    red_count: u64,
    green_count: u64,
    blue_count: u64,
}

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error> {
        let games = BufReader::new(File::open(input_path)?)
            .lines()
            .map(|line| Game::parse(&line?))
            .collect::<Result<Vec<_>, _>>()?;

        println!(
            "Sum of valid game ids: {}",
            games
                .iter()
                .filter(|game| game.is_valid())
                .map(|game| game.id)
                .sum::<u64>()
        );

        println!(
            "Sum of game cube \"powers\": {}",
            games
                .iter()
                .map(|game| game.power_of_min_cube_set())
                .sum::<u64>()
        );

        Ok(())
    }
}

impl Game {
    fn parse(s: &str) -> Result<Game, Error> {
        if let Some(caps) = GAME_RE.captures(s) {
            let mut game = Game::default();
            game.id = caps.name("id").unwrap().as_str().parse::<u64>()?;
            for r in caps.name("results").unwrap().as_str().split(';') {
                let mut result = GameResult::default();
                for caps in RESULT_RE.captures_iter(r) {
                    let count = caps.name("count").unwrap().as_str().parse::<u64>()?;
                    match caps.name("color").unwrap().as_str() {
                        "red" => result.red_count = count,
                        "green" => result.green_count = count,
                        "blue" => result.blue_count = count,
                        _ => unreachable!(),
                    }
                }
                game.results.push(result);
            }
            Ok(game)
        } else {
            Err(format_err!("Unparseable game: {}", s))
        }
    }

    fn is_valid(&self) -> bool {
        self.results.iter().all(|r| r.is_valid())
    }

    fn power_of_min_cube_set(&self) -> u64 {
        let r = self.results.iter().map(|r| r.red_count).max().unwrap();
        let g = self.results.iter().map(|r| r.green_count).max().unwrap();
        let b = self.results.iter().map(|r| r.blue_count).max().unwrap();
        r * g * b
    }
}

impl GameResult {
    fn is_valid(&self) -> bool {
        self.red_count <= MAX_RED && self.green_count <= MAX_GREEN && self.blue_count <= MAX_BLUE
    }
}

#[cfg(test)]
mod tests {}
