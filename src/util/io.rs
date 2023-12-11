use anyhow::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file_as_lines(path: &str) -> Result<Vec<String>, Error> {
    Ok(BufReader::new(File::open(path)?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?)
}

pub fn space_separated_numbers(s: &str) -> Result<Vec<u64>, Error> {
    Ok(s.split_whitespace()
        .map(|s| s.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?)
}
