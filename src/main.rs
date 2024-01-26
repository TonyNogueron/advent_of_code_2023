pub mod day1;
pub mod day12;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod utils;

fn main() {
    day5(); 
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

#[allow(dead_code)]
fn day3() {
    // Time taken p1: 36m 36s
    // Time taken p2: 1h 43m 5s
    let file_path = "./src/day3/p1.txt";
    match day3::day3part1(file_path) {
        Some(ans) => println!("Day 3 \n p1: {} \n p2: {}", ans.0, ans.1),
        _ => println!("Error"),
    }
}

#[allow(dead_code)]
fn day4() {
    // Time taken p1: 22m
    // Time taken p2: 45m
    let file_path = "./src/day4/p1.txt";
    match day4::day4part1(file_path) {
        Some(ans) => println!("Day 4\n p1: {}", ans),
        _ => println!("Error"),
    }
}

#[allow(dead_code)]
fn day12() {
    // Time taken p1: 3h 57m :(
    let file_path = "./src/day12/p1.txt";
    match day12::day12part1(file_path) {
        Some(ans) => println!("Day 12\n p1: {}", ans),
        _ => println!("Error"),
    }
}

#[allow(dead_code)]
fn day5() {
    // Time taken p1:
    let file_path = "./src/day5/test.txt";
    match day5::day5part1(file_path) {
        Some(ans) => println!("Day 5\n p1: {}", ans),
        _ => println!("Error"),
    }
}
