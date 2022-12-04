extern crate day01;

use day01::{process_buffer, read_file};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn part02(path: &str) {
    let buffer = read_file(path);
    let result = process_buffer(&buffer);
    let mut heap = BinaryHeap::new();

    for calorie in result {
        heap.push(Reverse(calorie));
        if heap.len() > 3 {
            heap.pop();
        }
    }

    println!(
        "{}",
        heap.into_iter()
            .map(|x| match x {
                Reverse(i) => i,
            })
            .sum::<i32>()
    );
}
