extern crate day02;

use day02::{read_file, Game, GameResult, Hand, Points, Strategy};

pub fn part01(path: &str) -> i32 {
    let data = read_file(path);
    let part1 = Part01::new();
    data.split("\n")
        .map(|x| {
            if x.len() > 0 {
                let mut split = x.split(" ").collect::<Vec<&str>>();
                let x2 = split.pop().unwrap();
                let x1 = split.pop().unwrap();

                part1.play(x1, x2.trim_end())
            } else {
                0
            }
        })
        .sum()
}

struct Part01 {}
impl Part01 {
    pub fn new() -> Self {
        Self {}
    }
}
impl Strategy for Part01 {
    fn played_hand(&self, _hand: &Hand, strategy: &str) -> Hand {
        match strategy {
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            _ => Hand::Unknown,
        }
    }
}

impl Game for Part01 {
    fn play(&self, hand: &str, strategy: &str) -> i32 {
        let hand1 = Hand::from_str(hand);
        let hand2 = self.played_hand(&hand1, strategy);
        let result = GameResult::from_hands(&hand1, &hand2);
        hand2.points() + result.points()
    }
}

#[test]
fn test_part01() {
    let part1 = Part01::new();
    assert_eq!(part1.play("A", "Y"), 8);
    assert_eq!(part1.play("B", "X"), 1);
    assert_eq!(part1.play("C", "Z"), 6);
    assert_eq!(part1.played_hand(&Hand::Rock, "Y"), Hand::Paper);
    assert_eq!(part1.played_hand(&Hand::Paper, "X"), Hand::Rock);
    assert_eq!(part1.played_hand(&Hand::Scissors, "Z"), Hand::Scissors);
}
