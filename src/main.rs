pub mod day1;

fn main() {
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
