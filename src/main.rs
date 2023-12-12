pub mod day1;
pub mod day2;
pub mod day3;
pub mod utils;

fn main() {
    day3()
}

#[allow(dead_code)]
fn day1() {
    let file_path = "./src/day1/day1.txt";
    match day1::part1(file_path) {
        Some(ans) => println!("Day 1 sum: {}", ans),
        _ => println!("Error"),
    }

    let file_path2 = "./src/day1/day1p2.txt";
    match day1::part2(file_path2) {
        Some(ans) => println!("Day 1 part 2 sum: {}", ans),
        _ => println!("Error"),
    }
}

#[allow(dead_code)]
fn day2() {
    // Time taken: 1h 12m
    let file_path = "./src/day2/p1.txt";
    match day2::day2part1(file_path) {
        Some(ans) => println!("Day 2: {}", ans),
        _ => println!("Error"),
    }

    // Time taken in part2: 5 minutes 30 seconds
    let file_path2 = "./src/day2/p1.txt";
    match day2::day2part2(file_path2) {
        Some(ans) => println!("Day 2 part 2 sum: {}", ans),
        _ => println!("Error"),
    }
}

fn day3() {
    // Time taken: 36m 36s
    let file_path = "./src/day3/p1.txt";
    match day3::day3part1(file_path) {
        Some(ans) => println!("Day 3: {}", ans),
        _ => println!("Error"),
    }
}
