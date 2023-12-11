use std::fs::File;
use std::io::{BufRead, BufReader};

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
