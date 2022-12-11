#[macro_use]
extern crate lazy_static;

use std::ops::{Range, RangeInclusive};

use regex::{Captures, Regex};

pub fn parse_row(row: &str) -> Option<(Assignment, Assignment)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?P<left_start>\d+)-(?P<left_end>\d+),(?P<right_start>\d+)-(?P<right_end>\d+)"
        )
        .unwrap();
    };
    let captures = RE.captures(row.trim());
    if let Some(captures) = captures {
        Some((
            Assignment::new(
                match_unwrap(&captures, "left_start"),
                match_unwrap(&captures, "left_end"),
            ),
            Assignment::new(
                match_unwrap(&captures, "right_start"),
                match_unwrap(&captures, "right_end"),
            ),
        ))
    } else {
        None
    }
}

fn match_unwrap(captures: &Captures, group: &str) -> i32 {
    captures
        .name(group)
        .unwrap()
        .as_str()
        .parse::<i32>()
        .unwrap()
}

pub struct Assignment {
    pub start: i32,
    pub end: i32,
}

impl Assignment {
    pub fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }

    pub fn to_range(&self) -> RangeInclusive<i32> {
        self.start..=self.end
    }
}

#[test]
fn test_parse_row() {
    let assignments = parse_row("2-4,6-8").unwrap();

    assert_eq!(assignments.0.start, 2);
    assert_eq!(assignments.0.end, 4);
    assert_eq!(assignments.1.start, 6);
    assert_eq!(assignments.1.end, 8);
}
