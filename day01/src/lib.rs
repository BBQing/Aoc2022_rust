use std::str::Split;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn parse_calories(lines: Split<&str>) -> i32 {
    lines.map(|x| x.parse::<i32>().unwrap_or_default()).sum()
}

pub fn process_buffer(buffer: &String) -> Vec<i32> {
    buffer
        .split("\n\n")
        .map(|x| x.split("\n"))
        .map(|x| parse_calories(x))
        .collect()
}

pub fn read_file(path: &str) -> String {
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();
    buffer

}