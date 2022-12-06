use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub fn read_file(path: &str) -> String {
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();
    buffer
}

#[test]
fn reads_testfile() {
    let testfile = "test.txt";
    let data = read_file(testfile);
    assert_eq!(&data[..1], &"A"[..1]);
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
    Unknown,
}

impl Hand {
    pub fn from_str(hand: &str) -> Self {
        match hand {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => Hand::Unknown,
        }
    }
}

impl Points for Hand {
    fn points(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
            Hand::Unknown => 0,
        }
    }
}

#[test]
fn test_hand() {
    assert_eq!(Hand::Paper.points(), 2);
    assert_eq!(Hand::from_str("A"), Hand::Rock);
    assert_eq!(Hand::from_str("B"), Hand::Paper);
    assert_eq!(Hand::from_str("C"), Hand::Scissors);
}
pub trait Points {
    fn points(&self) -> i32;
}

pub trait Strategy {
    fn played_hand(&self, hand: &Hand, strategy: &str) -> Hand;
}
#[derive(Debug, PartialEq)]
pub enum GameResult {
    Win,
    Draw,
    Lose,
}

impl GameResult {
    pub fn from_hands(hand: &Hand, hand_from_strategy: &Hand) -> GameResult {
        match (hand, hand_from_strategy) {
            (Hand::Unknown, _) => GameResult::Draw,
            (_, Hand::Unknown) => GameResult::Draw,
            (_, _) => match (hand.points() - hand_from_strategy.points()) % 3 {
                0 => GameResult::Draw,
                1 => GameResult::Lose,
                2 => GameResult::Win,
                -1 => GameResult::Win,
                -2 => GameResult::Lose,
                _ => GameResult::Draw,
            },
        }
    }
}
impl Points for GameResult {
    fn points(&self) -> i32 {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Lose => 0,
        }
    }
}
pub trait Game {
    fn play(&self, hand: &str, strategy: &str) -> i32;
}

#[test]
fn test_from_hands() {
    assert_eq!(
        GameResult::from_hands(&Hand::Rock, &Hand::Paper),
        GameResult::Win
    );
    assert_eq!(
        GameResult::from_hands(&Hand::Rock, &Hand::Rock),
        GameResult::Draw
    );
    assert_eq!(
        GameResult::from_hands(&Hand::Rock, &Hand::Scissors),
        GameResult::Lose
    );
    assert_eq!(
        GameResult::from_hands(&Hand::Paper, &Hand::Paper),
        GameResult::Draw
    );
    assert_eq!(
        GameResult::from_hands(&Hand::Paper, &Hand::Rock),
        GameResult::Lose
    );
    assert_eq!(
        GameResult::from_hands(&Hand::Paper, &Hand::Scissors),
        GameResult::Win
    );
    assert_eq!(
        GameResult::from_hands(&Hand::Scissors, &Hand::Paper),
        GameResult::Lose
    );
    assert_eq!(
        GameResult::from_hands(&Hand::Scissors, &Hand::Rock),
        GameResult::Win
    );
    assert_eq!(
        GameResult::from_hands(&Hand::Scissors, &Hand::Scissors),
        GameResult::Draw
    );
}