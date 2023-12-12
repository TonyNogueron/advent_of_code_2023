use std::collections::{HashMap, HashSet};

use crate::utils::read_file_lines;

pub fn day3part1(path: &str) -> Option<(i32, i32)> {
    match read_file_lines(path) {
        Some(lines) => Some(gear_ratio(lines)),
        None => {
            eprintln!("Error reading file or file not found: {}", path);
            return None;
        }
    }
}

fn is_char_valid(
    data: &Vec<Vec<char>>,
    gear_set: &mut HashSet<(usize, usize)>,
    x: usize,
    y: usize,
) -> bool {
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

    let mut is_valid = false;

    for direction in directions.iter() {
        let curr_x = x as i32 + direction.0;
        let curr_y = y as i32 + direction.1;
        if is_pos_valid(data, curr_x, curr_y) {
            if data[curr_y as usize][curr_x as usize] != '.'
                && !data[curr_y as usize][curr_x as usize].is_ascii_digit()
            {
                is_valid = true;
            }
            if data[curr_y as usize][curr_x as usize] == '*' {
                gear_set.insert((curr_x as usize, curr_y as usize));
            }
        }
    }
    is_valid
}

fn is_pos_valid(data: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    return x >= 0 && y >= 0 && x < data[0].len() as i32 && y < data.len() as i32;
}

fn gear_ratio(lines: Vec<String>) -> (i32, i32) {
    let data: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut gear_map: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    let mut curr_num = String::new();
    let mut is_curr_num_valid = false;

    let mut sum = 0;

    let mut x = 0;
    let mut y = 0;

    while y < data.len() {
        let mut gear_position_set: HashSet<(usize, usize)> = HashSet::new();
        while x < data[y].len() {
            if data[y][x].is_ascii_digit() {
                curr_num.push(data[y][x]);
                if is_char_valid(&data, &mut gear_position_set, x, y) {
                    is_curr_num_valid = true;
                }
            } else if curr_num.len() > 0 {
                gear_position_set.iter().for_each(|pos| {
                    gear_map
                        .entry(*pos)
                        .or_insert(Vec::new())
                        .push(curr_num.parse::<i32>().unwrap());
                });
                if is_curr_num_valid {
                    sum += curr_num.parse::<i32>().unwrap();
                }
                curr_num.clear();
                is_curr_num_valid = false;
                gear_position_set.clear();
            }
            x += 1;
        }
        if is_curr_num_valid {
            // In case the last char is a number
            sum += curr_num.parse::<i32>().unwrap();
            gear_position_set.iter().for_each(|pos| {
                gear_map
                    .entry(*pos)
                    .or_insert(Vec::new())
                    .push(curr_num.parse::<i32>().unwrap());
            });
            gear_position_set.clear();
            is_curr_num_valid = false;
            curr_num.clear();
        }
        x = 0;
        y += 1;
    }

    (sum, get_gear_ratio_sum(&gear_map))
}

fn get_gear_ratio_sum(gear_map: &HashMap<(usize, usize), Vec<i32>>) -> i32 {
    let mut sum = 0;
    for (_, value) in gear_map.iter() {
        if value.len() == 2 {
            sum += value[0] * value[1];
        }
    }
    sum
}
