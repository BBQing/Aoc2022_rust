extern crate day03;
extern crate utils;

use day03::priority;
use std::collections::HashSet;
use utils::read_file;

pub fn part01(path: &str) -> i32 {
    let buffer = read_file(path);
    buffer.split("\n").map(split_in_half).map(common_item).sum()
}

fn common_item(compartments: (&str, &str)) -> i32 {
    let mut left = HashSet::new();
    let mut right = HashSet::new();
    for char in compartments.0.chars() {
        left.insert(char);
    }
    for char in compartments.1.chars() {
        right.insert(char);
    }
    left.intersection(&right).take(1).map(priority).sum()
}

fn split_in_half(compartments: &str) -> (&str, &str) {
    let half = compartments.len() / 2;
    (&compartments[..half], &compartments[half..])
}

#[test]
fn test_part1() {
    assert_eq!(part01("test.txt"), 157)
}