use crate::advent::AdventSolver;
use crate::util::io;
use anyhow::{format_err, Error};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

#[derive(Default)]
pub struct Solver;

lazy_static! {
    static ref CARD_RE: Regex =
        Regex::new(r"^Card +(?P<id>\d+): (?P<winners>[0-9 ]+) \| (?P<picks>[0-9 ]+)$").unwrap();
}

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error> {
        let input = io::read_file_as_lines(input_path)?;
        let mut cards = input
            .iter()
            .map(|line| Card::parse(line))
            .collect::<Result<Vec<_>, _>>()?;

        println!(
            "Sum of card point values: {}",
            cards.iter().map(|c| c.point_value()).sum::<u64>()
        );

        Self::propagate_wins(&mut cards);
        println!(
            "Card count after propagation: {}",
            cards.iter().map(|c| c.copies).sum::<u64>()
        );

        Ok(())
    }
}

impl Solver {
    fn propagate_wins(cards: &mut Vec<Card>) {
        for i in 0..cards.len() {
            for j in 1..=cards[i].win_count() as usize {
                cards[i + j].copies += cards[i].copies;
            }
        }
    }
}

struct Card {
    copies: u64,
    winners: HashSet<u64>,
    picks: HashSet<u64>,
}

impl Card {
    fn parse(line: &str) -> Result<Card, Error> {
        if let Some(caps) = CARD_RE.captures(line) {
            let winners = Self::parse_numbers(&caps["winners"])?;
            let picks = Self::parse_numbers(&caps["picks"])?;
            Ok(Card {
                copies: 1,
                winners,
                picks,
            })
        } else {
            Err(format_err!("Could not parse card: {}", line))
        }
    }

    fn parse_numbers(numbers: &str) -> Result<HashSet<u64>, Error> {
        Ok(numbers
            .split_whitespace()
            .map(|w| w.parse::<u64>())
            .collect::<Result<HashSet<_>, _>>()?)
    }

    fn point_value(&self) -> u64 {
        let count = self.win_count();
        if count == 0 {
            0
        } else {
            2u64.pow((count - 1) as u32)
        }
    }

    fn win_count(&self) -> u64 {
        self.winners
            .iter()
            .filter(|w| self.picks.contains(w))
            .count() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::{Card, Solver};
    use lazy_static::lazy_static;

    lazy_static! {
        static ref EX_IN: Vec<String> = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ];
    }

    #[test]
    fn test_example_cards() {
        let cards = EX_IN
            .iter()
            .map(|line| Card::parse(line).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(8, cards[0].point_value());
        assert_eq!(2, cards[1].point_value());
        assert_eq!(2, cards[2].point_value());
        assert_eq!(1, cards[3].point_value());
        assert_eq!(0, cards[4].point_value());
        assert_eq!(0, cards[5].point_value());
    }

    #[test]
    fn test_part2_copy_rule() {
        let mut cards = EX_IN
            .iter()
            .map(|line| Card::parse(line).unwrap())
            .collect::<Vec<_>>();

        Solver::propagate_wins(&mut cards);
        assert_eq!(30u64, cards.iter().map(|c| c.copies).sum());
    }
}
