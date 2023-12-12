use crate::utils::read_file_lines;

pub fn day3part1(path: &str) -> Option<i32> {
    match read_file_lines(path) {
        Some(lines) => Some(numbers_in_engine(lines)),
        None => {
            eprintln!("Error reading file or file not found: {}", path);
            return None;
        }
    }
}

pub fn day3part2(path: &str) -> Option<i32> {
    match read_file_lines(path) {
        Some(_lines) => Some(0),
        None => {
            eprintln!("Error reading file or file not found: {}", path);
            return None;
        }
    }
}

fn numbers_in_engine(lines: Vec<String>) -> i32 {
    let data: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let mut curr_num = String::new();
    let mut is_curr_num_valid = false;

    let mut sum = 0;

    let mut x = 0;
    let mut y = 0;

    while y < data.len() {
        while x < data[y].len() {
            if data[y][x].is_ascii_digit() {
                curr_num.push(data[y][x]);
                if is_char_valid(&data, x, y) {
                    is_curr_num_valid = true;
                }
            } else {
                if is_curr_num_valid {
                    sum += curr_num.parse::<i32>().unwrap();
                }
                curr_num.clear();
                is_curr_num_valid = false;
            }
            x += 1;
        }
        if is_curr_num_valid && curr_num.len() > 0 {
            // In case the last char is a number
            sum += curr_num.parse::<i32>().unwrap();
            is_curr_num_valid = false;
            curr_num.clear();
        }
        x = 0;
        y += 1;
    }

    sum
}

fn is_char_valid(data: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let directions = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for direction in directions.iter() {
        let curr_x = x as i32 + direction.0;
        let curr_y = y as i32 + direction.1;
        if is_pos_valid(data, curr_x, curr_y) {
            if data[curr_y as usize][curr_x as usize] != '.'
                && !data[curr_y as usize][curr_x as usize].is_ascii_digit()
            {
                return true;
            }
        }
    }
    false
}

fn is_pos_valid(data: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    return x >= 0 && y >= 0 && x < data[0].len() as i32 && y < data.len() as i32;
}
