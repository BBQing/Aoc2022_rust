extern crate day01;

use day01::{process_buffer, read_file};

pub fn part01(path: &str) {
    let buffer = read_file(path);
    let result = process_buffer(&buffer).into_iter().max();

    if let Some(r) = result {
        println!("{}", r);
    }
}
