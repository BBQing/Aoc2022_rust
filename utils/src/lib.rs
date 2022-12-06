use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub fn read_file(path: &str) -> String {
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();
    buffer

}