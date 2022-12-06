extern crate day02;

use day02::{read_file, Game, GameResult, Hand, Points, Strategy};

pub fn part02(path: &str) -> i32 {
    let data = read_file(path);
    let part2 = Part02::new();
    data.split("\n")
        .map(|x| {
            if x.len() > 0 {
                let mut split = x.split(" ").collect::<Vec<&str>>();
                let x2 = split.pop().unwrap();
                let x1 = split.pop().unwrap();

                part2.play(x1, x2.trim_end())
            } else {
                0
            }
        })
        .sum()
}

struct Part02 {}
impl Part02 {
    pub fn new() -> Self {
        Self {}
    }
}
impl Strategy for Part02 {
    fn played_hand(&self, hand: &Hand, strategy: &str) -> Hand {
        match (hand, strategy) {
            (_,"Y") => hand.clone(),
            (_,"X") => match hand {
                Hand::Rock => Hand::Scissors,
                Hand::Paper => Hand::Rock,
                Hand::Scissors => Hand::Paper,
                _ => Hand::Unknown
            }
            (_,"Z") => match hand {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissors,
                Hand::Scissors => Hand::Rock,
                _ => Hand::Unknown
            }
            _ => Hand::Unknown,
        }
    }

}

impl Game for Part02 {
    fn play(&self, hand: &str, strategy: &str) -> i32 {
        let hand1 = Hand::from_str(hand);
        let hand2 = self.played_hand(&hand1, strategy);
        let result = GameResult::from_hands(&hand1, &hand2);
        hand2.points() + result.points()
    }
}

#[test]
fn test_part02() {
    let part1 = Part02::new();
    assert_eq!(part1.play("A", "Y"), 4);
    assert_eq!(part1.play("B", "X"), 1);
    assert_eq!(part1.play("C", "Z"), 7);
    assert_eq!(part1.played_hand(&Hand::Rock, "Y"), Hand::Rock);
    assert_eq!(part1.played_hand(&Hand::Paper, "X"), Hand::Rock);
    assert_eq!(part1.played_hand(&Hand::Scissors, "Z"), Hand::Rock);
}
