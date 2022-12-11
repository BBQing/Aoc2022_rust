extern crate day03;
extern crate utils;

use day03::priority;
use itertools::{Chunk, Itertools};
use std::collections::HashSet;
use std::str::Split;
use utils::read_file;

pub fn part02(path: &str) -> i32 {
    let buffer = read_file(path);
    buffer
        .trim()
        .split("\n")
        .chunks(3)
        .into_iter()
        .map(process_batch)
        .sum::<i32>()
}

#[test]
fn test_part2() {
    assert_eq!(part02("test.txt"), 70);
}

fn process_batch(batch: Chunk<Split<&str>>) -> i32 {
    let single_item = batch
        .map(|x| {
            let mut hash = HashSet::new();
            for char in x.chars() {
                hash.insert(char);
            }
            hash
        })
        .into_iter()
        .reduce(|acc, item| HashSet::<char>::from_iter(acc.intersection(&item).map(|x| *x)))
        .unwrap()
        .drain()
        .take(1)
        .next()
        .unwrap();
    priority(&single_item)
}
