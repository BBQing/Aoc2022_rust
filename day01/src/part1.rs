extern crate day01;
extern crate utils;

use day01::{process_buffer};
use utils::read_file;

pub fn part01(path: &str) {
    let buffer = read_file(path);
    let result = process_buffer(&buffer).into_iter().max();

    if let Some(r) = result {
        println!("{}", r);
    }
}
