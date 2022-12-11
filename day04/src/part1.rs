extern crate day04;
extern crate utils;

use day04::{parse_row, Assignment};
use std::str::Split;
use utils::read_file;

pub fn part01(path: &str) -> i32 {
    let buffer = read_file(path);
    buffer
        .trim()
        .split("\n")
        .map(parse_row)
        .map(contains)
        .filter(|x| *x)
        .count() as i32
}

#[test]
fn test_part1() {
    assert_eq!(part01("test.txt"), 2)
}
fn contains(assignments: Option<(Assignment, Assignment)>) -> bool {
    fn _contains(left: &Assignment, right: &Assignment) -> bool {
        (left.start - right.start) * (left.end - right.end) <= 0
    }

    if let Some((left, right)) = assignments {
        _contains(&left, &right)
    } else {
        false
    }
}
