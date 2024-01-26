use crate::utils::read_file;

pub fn day5part1(path: &str) -> Option<i32> {
    match read_file(path) {
        Some(contents) => Some(part1(contents)),
        None => {
            eprintln!("Error reading file or file not found: {}", path);
            return None;
        }
    }
}

fn part1(contents: String) -> i32 {
    let blocks: Vec<String> = contents.split("\n\n").map(|s| s.to_string()).collect();

    let seeds: Vec<i32> = blocks[0].split(":").nth(1).unwrap().split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();

    println!("seeds: {:?}", seeds);

    for block in blocks {
        println!("{}", block);
    }
    0
}

/*
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
*/
