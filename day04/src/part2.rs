extern crate day04;
extern crate utils;

use day04::{parse_row, Assignment};
use std::str::Split;
use utils::read_file;

pub fn part02(path: &str) -> i32 {
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
fn test_part2() {
    assert_eq!(part02("test.txt"), 4);
}

fn contains(row: Option<(Assignment, Assignment)>) -> bool {
    if let Some((left, right)) = row {
        _contains(&left, &right)
    } else {
        false
    }
}

fn _contains(left: &Assignment, right: &Assignment) -> bool {
    let right_range = right.to_range();
    left.to_range().any(|x| right_range.contains(&x))
}

#[test]
fn test_contains() {
    let ass1 = Assignment::new(2, 6);
    let ass2 = Assignment::new(4, 8);
    assert_eq!(_contains(&ass1, &ass2), true)
}
