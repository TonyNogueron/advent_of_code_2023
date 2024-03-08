use std::{collections::HashMap, u64::MAX};

use crate::utils::read_file;

pub fn day5part1(path: &str) -> Option<u64> {
    match read_file(path) {
        Some(contents) => Some(part1(contents)),
        None => {
            eprintln!("Error reading file or file not found: {}", path);
            return None;
        }
    }
}

#[derive(Debug)]
struct Item {
    dst: u64,
    src: u64,
    range: u64,
}

#[derive(Debug)]
struct Block {
    from: String,
    to: String,
    items: Vec<Item>,
}

fn part1(contents: String) -> u64 {
    let blocks: Vec<String> = contents.split("\n\n").map(|s| s.to_string()).collect();
    let seeds = parse_seeds(&blocks[0]);

    let mut blocks_parsed: Vec<Block> = vec![];

    for i in 1..blocks.len() {
        blocks_parsed.push(parse_block(&blocks[i]));
    }

    // println!("blocks: {:?}", blocks_parsed);

    let mut min: u64 = MAX;

    let mut block_map: HashMap<String, u64> = HashMap::new();

    for i in 0..blocks_parsed.len() {
        if !block_map.contains_key(&blocks_parsed[i].from) {
            block_map.insert(blocks_parsed[i].from.clone(), i as u64);
        }
    }

    for seed in seeds {
        let converted = walk_map(seed, "seed".to_string(), &block_map, &blocks_parsed);
        // println!("Converted seed: {} to {}", seed, converted);
        if converted < min {
            min = converted;
        }
    }

    min
}

fn walk_map(
    value: u64,
    from: String,
    block_map: &HashMap<String, u64>,
    blocks_parsed: &Vec<Block>,
) -> u64 {
    let mut transformed_value: u64 = value;

    if block_map.contains_key(&from) {
        let block = &blocks_parsed[block_map[&from] as usize];
        for item in &block.items {
            if value >= item.src && value <= item.src + item.range {
                transformed_value = item.dst + (value - item.src);
                return walk_map(
                    transformed_value,
                    block.to.clone(),
                    block_map,
                    blocks_parsed,
                );
            }
        }
        return walk_map(value, block.to.clone(), block_map, blocks_parsed);
    }
    transformed_value
}

fn parse_block(block: &str) -> Block {
    let lines: Vec<String> = block.split("\n").map(|s| s.to_string()).collect();

    let (from, to) = parse_from_to(&lines[0]);
    let mut items: Vec<Item> = vec![];

    for i in 1..lines.len() {
        if lines[i] == "" {
            continue;
        }
        items.push(parse_item(&lines[i]));
    }

    let b: Block = Block { from, to, items };
    b
}

fn parse_from_to(line: &str) -> (String, String) {
    let aux: String = line.split(" ").next().unwrap().to_string();
    let from_to: Vec<String> = aux.split("-").map(|s| s.to_string()).collect();
    (from_to[0].clone(), from_to[2].clone())
}

fn parse_item(line: &str) -> Item {
    let item: Vec<u64> = line
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("Error parsing item to number"))
        .collect();
    Item {
        dst: item[0],
        src: item[1],
        range: item[2],
    }
}

fn parse_seeds(line: &str) -> Vec<u64> {
    let seeds: Vec<u64> = line
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("Error parsing seed to number"))
        .collect();
    seeds
}
