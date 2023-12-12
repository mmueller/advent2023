use crate::advent::AdventSolver;
use crate::util::io;
use anyhow::Error;
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error> {
        let input = io::read_file_as_lines(input_path)?;
        let mut hands = input
            .iter()
            .map(|s| (CamelHand::from(&s[0..5]), s[6..].parse::<u64>().unwrap()))
            .collect::<Vec<(CamelHand, u64)>>();
        println!("Total winnings: {}", Self::total_winnings(&hands));
        for (ref mut hand, _) in hands.iter_mut() {
            hand.jokers_wild();
        }
        println!("Total winnings: {}", Self::total_winnings(&hands));
        Ok(())
    }
}

impl Solver {
    fn total_winnings(hands: &Vec<(CamelHand, u64)>) -> u64 {
        let mut sorted: Vec<(CamelHand, u64)> = hands.clone();
        sorted.sort();
        sorted
            .iter()
            .enumerate()
            .fold(0, |total, (i, (_hand, bet))| total + bet * (i as u64 + 1))
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum CamelHandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CamelHand {
    cards: Vec<char>,
    jokers: bool,
}

impl CamelHand {
    // Joker-enabled hand type calculation
    fn hand_type(&self) -> CamelHandType {
        let joker_count = self.cards.iter().filter(|&&c| c == 'J').count();
        if !self.jokers || joker_count == 0 || joker_count == 5 {
            Self::base_hand_type(&self.cards)
        } else {
            // Hand contains 1 to 4 jokers and jokers are wild.
            let normal_cards = self
                .cards
                .iter()
                .map(|c| *c)
                .filter(|&c| c != 'J')
                .collect::<Vec<_>>();
            // Try replacing jokers with a copy of each of the other cards in the hand, and see
            // which results in the best hand value.
            normal_cards
                .iter()
                .map(|c| [normal_cards.clone(), [*c].repeat(joker_count)].concat())
                .map(|wild_hand| Self::base_hand_type(&wild_hand))
                .max()
                .unwrap()
        }
    }

    // Non-joker-enabled hand type calculation
    fn base_hand_type(cards: &Vec<char>) -> CamelHandType {
        let mut sorted = cards.clone();
        sorted.sort();
        let at_least_three_of_a_kind =
            sorted[0] == sorted[2] || sorted[1] == sorted[3] || sorted[2] == sorted[4];
        let at_least_four_of_a_kind = sorted[0] == sorted[3] || sorted[1] == sorted[4];
        let mut deduped = sorted.to_vec();
        deduped.dedup();
        match deduped.len() {
            5 => CamelHandType::HighCard,
            4 => CamelHandType::OnePair,
            3 => {
                if at_least_three_of_a_kind {
                    CamelHandType::ThreeOfAKind
                } else {
                    CamelHandType::TwoPair
                }
            }
            2 => {
                if at_least_four_of_a_kind {
                    CamelHandType::FourOfAKind
                } else {
                    CamelHandType::FullHouse
                }
            }
            1 => CamelHandType::FiveOfAKind,
            _ => panic!(),
        }
    }

    fn card_value(&self, card: char) -> u64 {
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => {
                if self.jokers {
                    1
                } else {
                    11
                }
            }
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            _ => panic!(),
        }
    }

    fn to_string(&self) -> String {
        self.cards.iter().collect::<String>()
    }

    fn jokers_wild(&mut self) {
        self.jokers = true;
    }
}

impl From<&str> for CamelHand {
    fn from(s: &str) -> CamelHand {
        CamelHand {
            cards: s.chars().collect::<Vec<_>>(),
            jokers: false,
        }
    }
}

impl Ord for CamelHand {
    fn cmp(&self, other: &Self) -> Ordering {
        (
            self.hand_type(),
            self.card_value(self.cards[0]),
            self.card_value(self.cards[1]),
            self.card_value(self.cards[2]),
            self.card_value(self.cards[3]),
            self.card_value(self.cards[4]),
        )
            .cmp(&(
                other.hand_type(),
                self.card_value(other.cards[0]),
                self.card_value(other.cards[1]),
                self.card_value(other.cards[2]),
                self.card_value(other.cards[3]),
                self.card_value(other.cards[4]),
            ))
    }
}

impl PartialOrd for CamelHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_hand_winnings() {
        let hands = vec![
            (CamelHand::from("32T3K"), 765),
            (CamelHand::from("T55J5"), 684),
            (CamelHand::from("KK677"), 28),
            (CamelHand::from("KTJJT"), 220),
            (CamelHand::from("QQQJA"), 483),
        ];
        assert_eq!(6440, Solver::total_winnings(&hands));
    }

    #[test]
    fn test_example_hand_sorting() {
        let mut hands = vec![
            CamelHand::from("32T3K"),
            CamelHand::from("T55J5"),
            CamelHand::from("KK677"),
            CamelHand::from("KTJJT"),
            CamelHand::from("QQQJA"),
        ];
        hands.sort();
        assert_eq!("32T3K".to_string(), hands[0].to_string());
        assert_eq!("KTJJT".to_string(), hands[1].to_string());
        assert_eq!("KK677".to_string(), hands[2].to_string());
        assert_eq!("T55J5".to_string(), hands[3].to_string());
        assert_eq!("QQQJA".to_string(), hands[4].to_string());
    }

    #[test]
    fn test_example_hand_sorting_with_jokers() {
        let mut hands = vec![
            CamelHand::from("32T3K"),
            CamelHand::from("T55J5"),
            CamelHand::from("KK677"),
            CamelHand::from("KTJJT"),
            CamelHand::from("QQQJA"),
        ];
        hands.iter_mut().for_each(|h| h.jokers_wild());
        hands.sort();
        assert_eq!("32T3K".to_string(), hands[0].to_string());
        assert_eq!("KK677".to_string(), hands[1].to_string());
        assert_eq!("T55J5".to_string(), hands[2].to_string());
        assert_eq!("QQQJA".to_string(), hands[3].to_string());
        assert_eq!("KTJJT".to_string(), hands[4].to_string());
    }

    #[test]
    fn test_hand_types() {
        assert_eq!(
            CamelHandType::HighCard,
            CamelHand::from("2K4T6").hand_type()
        );
        assert_eq!(CamelHandType::OnePair, CamelHand::from("4QA2Q").hand_type());
        assert_eq!(CamelHandType::TwoPair, CamelHand::from("AA655").hand_type());
        assert_eq!(
            CamelHandType::ThreeOfAKind,
            CamelHand::from("9299Q").hand_type()
        );
        assert_eq!(
            CamelHandType::FullHouse,
            CamelHand::from("56565").hand_type()
        );
        assert_eq!(
            CamelHandType::FourOfAKind,
            CamelHand::from("JJJ3J").hand_type()
        );
        assert_eq!(
            CamelHandType::FiveOfAKind,
            CamelHand::from("77777").hand_type()
        );
    }
}
