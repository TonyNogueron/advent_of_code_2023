use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub fn read_file_lines(path: &str) -> Option<Vec<String>> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return None,
    };

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    if lines.is_empty() {
        None
    } else {
        Some(lines)
    }
}

pub fn read_file(path: &str) -> Option<String> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return None,
    };

    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    match reader.read_to_string(&mut contents) {
        Ok(_) => Some(contents),
        Err(_) => None,
    }
}
