use crate::utils::read_file_lines;
use std::collections::HashMap;

pub fn day12part1(path: &str) -> Option<i32> {
    match read_file_lines(path) {
        Some(lines) => Some(count_possible_arrangements(lines)),
        None => {
            eprintln!("Error reading file or file not found: {}", path);
            return None;
        }
    }
}

fn count_possible_arrangements(lines: Vec<String>) -> i32 {
    let mut ans = 0;
    let mut dp: HashMap<String, i32> = HashMap::new();
    for line in lines {
        let record: Vec<&str> = line.split(' ').collect();
        let condition = record[0];

        let valid: Vec<i32> = record[1]
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        ans += recursive_count(condition, &valid, &mut dp);
    }
    ans
}

fn recursive_count(condition: &str, valid: &[i32], dp: &mut HashMap<String, i32>) -> i32 {
    if condition.is_empty() {
        // If we finished with a valid solution return 1
        return if valid.is_empty() { 1 } else { 0 };
    }

    if valid.is_empty() {
        // If we finished the valid set but there is still a block left, this is not a valid solution
        return if condition.contains('#') { 0 } else { 1 };
    }

    let key = get_key_from_tuple((condition, valid)); // We can check if we already calculated this
    if let Some(&ans) = dp.get(&key) {
        return ans;
    }

    let mut result = 0;

    if condition.starts_with('.') || condition.starts_with('?') {
        // This is the case where we skip a dot or decide to replace the '?' with a dot, so we just continue
        result += recursive_count(&condition[1..], &valid, dp);
    }

    if condition.starts_with('#') || condition.starts_with('?') {
        if valid[0] <= condition.chars().count() as i32 // If we need more '#' than we have left is not valid
            && !condition[0..valid[0] as usize].contains('.') // If we find a '.' in the block that we need to fill, this is not valid
            && (valid[0] == condition.chars().count() as i32 // If we need to fill the whole block, it means we can continue
                || !condition[valid[0] as usize..].starts_with('#'))
        // If the condition continues, then if we are able to finish the block, we can continue
        {
            // This is the case where we decide to replace the '?' with a '#', so we just continue
            // We were able to fill the block that was required so we remove it from the valid set and move on
            if (valid[0] as usize + 1) < condition.len() {
                result += recursive_count(&condition[valid[0] as usize + 1..], &valid[1..], dp);
            } else {
                result += recursive_count("", &valid[1..], dp);
            }
        }
    }

    dp.insert(key, result);
    result
}

fn get_key_from_tuple(key: (&str, &[i32])) -> String {
    let valid_str: String = key.1.iter().map(|x| x.to_string()).collect();
    key.0.to_string() + &valid_str
}

// Ended up not using this :/
// fn condition_matches_validation(condition: &str, valid_sets: &Vec<i32>) -> bool {
//     let mut broken_sets: Vec<i32> = vec![];
//     let broken_num = condition.chars().fold(0, |acc, c| {
//         if c == '#' {
//             acc + 1
//         } else {
//             if acc > 0 {
//                 broken_sets.push(acc);
//             }
//             0
//         }
//     });

//     // Check if the last character is '#'
//     if broken_num > 0 {
//         broken_sets.push(broken_num);
//     }

//     *valid_sets == broken_sets
// }
