use crate::utils::read_file_lines;

pub fn day2part1(path: &str) -> Option<i32> {
    match read_file_lines(path) {
        Some(lines) => Some(
            lines
                .iter()
                .map(|line| is_gameset_possible(line))
                .sum::<i32>(),
        ),
        None => {
            eprintln!("Error reading file or file not found: {}", path);
            return None;
        }
    }
}

pub fn day2part2(path: &str) -> Option<i32> {
    match read_file_lines(path) {
        Some(lines) => Some(lines.iter().map(|line| power_of_set(line)).sum::<i32>()),
        None => {
            eprintln!("Error reading file or file not found: {}", path);
            return None;
        }
    }
}

fn is_gameset_possible(line: &str) -> i32 {
    // Max red=12, max green=13, max blue=14

    let line_split: Vec<&str> = line.split(":").collect();
    let game_id = line_split[0].replace("Game ", "");
    let game_sets: Vec<&str> = line_split[1].split(";").collect();

    for set in game_sets.iter() {
        let set_split: Vec<&str> = set.split(",").collect();
        for color in set_split.iter() {
            let color_split: Vec<&str> = color.split_whitespace().collect();
            let color_count = color_split[0].parse::<i32>().unwrap();
            let color_name = color_split[1].trim();
            match color_name {
                "red" => {
                    if color_count > 12 {
                        return 0;
                    }
                }
                "green" => {
                    if color_count > 13 {
                        return 0;
                    }
                }
                "blue" => {
                    if color_count > 14 {
                        return 0;
                    }
                }
                _ => println!("Unknown color: {}", color_name),
            }
        }
    }
    game_id.parse::<i32>().unwrap()
}

fn power_of_set(line: &str) -> i32 {
    let line_split: Vec<&str> = line.split(":").collect();
    let game_sets: Vec<&str> = line_split[1].split(";").collect();

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for set in game_sets.iter() {
        let set_split: Vec<&str> = set.split(",").collect();
        for color in set_split.iter() {
            let color_split: Vec<&str> = color.split_whitespace().collect();
            let color_count = color_split[0].parse::<i32>().unwrap();
            let color_name = color_split[1].trim();
            match color_name {
                "red" => {
                    if color_count > max_red {
                        max_red = color_count;
                    }
                }
                "green" => {
                    if color_count > max_green {
                        max_green = color_count;
                    }
                }
                "blue" => {
                    if color_count > max_blue {
                        max_blue = color_count;
                    }
                }
                _ => println!("Unknown color: {}", color_name),
            }
        }
    }
    max_red * max_green * max_blue
}
