use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fmt::format;
use std::fs;
use std::process;

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: String,
    value: i64,
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
enum PokerHands {
    OneOfAKind = 1,
    TwoOfAKind = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

impl Hand {
    fn new(raw_data: &str) -> Hand {
        let split_data: Vec<&str> = raw_data.split_whitespace().collect();
        let cards_raw = split_data.get(0).unwrap();
        let cards = cards_raw.to_string();
        let value: i64 = split_data.get(1).unwrap().parse().unwrap();
        return Hand { cards, value };
    }

    fn rank(&self) -> PokerHands {
        let mut card_map: HashMap<char, i64> = HashMap::new();
        for card in self.cards.chars() {
            let new_value = match card_map.get(&card) {
                Some(n) => n + 1,
                None => 1,
            };
            card_map.insert(card, new_value);
        }
        let j_value = match card_map.remove(&'J') {
            Some(n) => n,
            None => 0,
        };
        let card_count = match card_map.values().max() {
            Some(v) => v + j_value,
            None => j_value,
        };
        let value = match card_count {
            2 => {
                if card_map.values().filter(|x| **x == 2).count() == 2 {
                    PokerHands::TwoPair
                } else {
                    PokerHands::TwoOfAKind
                }
            }
            3 => {
                if card_map.values().filter(|x| **x == 2).count() as i64 == 1 + j_value {
                    PokerHands::FullHouse
                } else {
                    PokerHands::ThreeOfAKind
                }
            }
            4 => PokerHands::FourOfAKind,
            5 => PokerHands::FiveOfAKind,
            1 => PokerHands::OneOfAKind,
            i => panic!(
                "{} This should not be a possible number of cards in a hand!",
                i
            ),
        };
        return value;
    }
    fn nth(&self, i: usize) -> char {
        return self.cards.chars().nth(i).unwrap();
    }
}

fn card_rank(card: &char) -> i64 {
    let card_ordering = vec![
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];
    let mut card_ordering = card_ordering.iter();
    return card_ordering.position(|c| c == card).unwrap() as i64;
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        let mut ord = self.rank().cmp(&other.rank());
        let mut i = 0;
        while ord == Ordering::Equal && i < 5 {
            ord = card_rank(&self.nth(i)).cmp(&card_rank(&other.nth(i)));
            i += 1;
        }
        return ord;
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = match args.get(1) {
        None => {
            println!("Please provide a file path!");
            process::exit(1);
        }
        Some(path) => path,
    };
    let raw_cards_and_score = fs::read_to_string(file_path)
        .expect(&format(format_args!("Cannot find file {}", file_path)));
    let mut cards: Vec<Hand> = raw_cards_and_score
        .lines()
        .map(|hand| Hand::new(hand))
        .collect();
    cards.sort();
    let mut score = 0;
    for (i, hand) in cards.iter().enumerate() {
        score += (i as i64 + 1) * hand.value;
    }
    dbg!(score);
}

#[test]
fn hand_rank() {
    let file_path = "./src/test3.txt";
    let raw_cards_and_score = fs::read_to_string(file_path)
        .expect(&format(format_args!("Cannot find file {}", file_path)));
    let cards: Vec<Hand> = raw_cards_and_score
        .lines()
        .map(|hand| Hand::new(hand))
        .collect();
    for card in cards {
        assert!(
            card.rank() as i64 == card.value,
            "Hand {} does not return value {}, instead {}",
            card.cards,
            card.value,
            card.rank() as i64
        );
    }
}
