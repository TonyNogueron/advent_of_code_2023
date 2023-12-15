use crate::utils::read_file_lines;

pub fn day4part1(path: &str) -> Option<i32> {
    match read_file_lines(path) {
        Some(lines) => Some(part2(lines)),
        None => {
            eprintln!("Error reading file or file not found: {}", path);
            return None;
        }
    }
}

/*
// part1
fn calculate_points(line: &str) -> i32 {
    let mut points = 0;
    let mut card = line.split("|");

    let winning: Vec<&str> = card
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect();

    card.next()
        .unwrap()
        .split_whitespace()
        .into_iter()
        .for_each(|x| {
            if winning.contains(&x) {
                if points > 0 {
                    points *= 2;
                } else {
                    points = 1;
                }
            }
        });
    points
}
*/

struct Card {
    pub copies: i32,
    pub points: i32,
}

fn part2(lines: Vec<String>) -> i32 {
    let mut cards: Vec<Card> = Vec::new();
    for line in lines {
        calculate_cards_p2(&line, &mut cards);
    }

    let num_cards = cards.len();

    for i in 0..num_cards {
        for _ in 0..cards[i].copies {
            if cards[i].points > 0 {
                for j in 0..cards[i].points {
                    if (i + 1 + j as usize) < num_cards {
                        cards[i + 1 + j as usize].copies += 1;
                    }
                }
            }
        }
    }

    cards.iter().fold(0, |acc, card| acc + card.copies)
}

fn calculate_cards_p2(line: &str, cards: &mut Vec<Card>) {
    let mut points = 0;
    let mut card = line.split("|");

    let winning: Vec<&str> = card
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect();

    card.next()
        .unwrap()
        .split_whitespace()
        .into_iter()
        .for_each(|x| {
            if winning.contains(&x) {
                points += 1;
            }
        });

    cards.push(Card { copies: 1, points });
}
