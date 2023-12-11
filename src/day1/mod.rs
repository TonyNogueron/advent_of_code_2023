use crate::utils::read_file_lines;

fn get_digits_from_line(line: &str) -> i32 {
    let chars: Vec<char> = line.chars().collect();
    let mut digit1 = 0;
    let mut digit2 = 0;

    let mut i = 0;
    let mut j = chars.len() - 1;
    let mut first_digit = false;
    let mut last_digit = false;
    while i <= j {
        if first_digit == false {
            if chars[i].is_digit(10) {
                first_digit = true;
                digit1 = chars[i].to_digit(10).unwrap() as i32;
            } else {
                i += 1;
            }
        }
        if last_digit == false {
            if chars[j].is_digit(10) {
                last_digit = true;
                digit2 = chars[j].to_digit(10).unwrap() as i32;
            } else {
                j -= 1;
            }
        }
        if first_digit && last_digit {
            break;
        }
    }
    digit1 * 10 + digit2
}

fn get_digits_part_2(line: &str) -> i32 {
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    let mut j = line.len() - 1;

    let mut digit1 = 0;
    let mut digit2 = 0;

    let mut first_digit = false;
    let mut last_digit = false;

    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    while i <= j {
        if !first_digit {
            if chars[i].is_ascii_digit() {
                first_digit = true;
                digit1 = chars[i].to_digit(10).unwrap() as i32;
            } else {
                for (index, number) in numbers.iter().enumerate() {
                    if i + number.len() > chars.len() {
                        continue;
                    }
                    if line[i..i + number.len()] == **number {
                        first_digit = true;
                        digit1 = index as i32 + 1;
                        break;
                    }
                }
                i += 1;
            }
        }

        if !last_digit {
            if chars[j].is_ascii_digit() {
                last_digit = true;
                digit2 = chars[j].to_digit(10).unwrap() as i32;
            } else {
                for (index, number) in numbers.iter().enumerate() {
                    if (j as i32 - number.len() as i32) < 0 {
                        continue;
                    }
                    if line[j - number.len() + 1..=j] == **number {
                        last_digit = true;
                        digit2 = index as i32 + 1;
                        break;
                    }
                }
                j -= 1;
            }
        }

        if first_digit && last_digit {
            break;
        }
    }
    digit1 * 10 + digit2
}

pub fn part2(path: &str) -> Option<i32> {
    match read_file_lines(path) {
        Some(lines) => {
            let mut sum = 0;
            for line in lines {
                sum += get_digits_part_2(&line);
            }
            return Some(sum);
        }
        None => {
            eprintln!("Error reading file or file not found: {}", path);
            return None;
        }
    }
}

pub fn part1(path: &str) -> Option<i32> {
    match read_file_lines(path) {
        Some(lines) => {
            return Some(
                lines
                    .iter()
                    .map(|line| get_digits_from_line(line))
                    .sum::<i32>(),
            );
        }
        None => {
            eprintln!("Error reading file or file not found: {}", path);
            return None;
        }
    }
}
