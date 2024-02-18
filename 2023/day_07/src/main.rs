use std::{cmp::Ordering, collections::HashMap};

use lazy_static::lazy_static;
use library::{get_filename_arg, get_lines};

fn main() {
    let file_name = get_filename_arg();
    let lines = get_lines(file_name.as_str());
    let mut hands = get_hands(lines);

    println!("Part 1: {}", get_sum(&mut hands, part_one_compare_cards));
    println!("Part 2: {}", get_sum(&mut hands, part_two_compare_cards));
}

struct Hand {
    cards: String,
    bid: u64,
}

fn get_hands(lines: Vec<String>) -> Vec<Hand> {
    let mut hands = vec![];
    for line in lines {
        let hand: Vec<&str> = line.split(" ").collect();
        hands.push(Hand {
            cards: hand[0].to_string(),
            bid: hand[1].parse().expect("Failed to parse bid into number"),
        });
    }

    hands
}

fn get_hand_histogram(cards: &String) -> HashMap<char, u8> {
    let mut histogram = HashMap::from([
        ('A', 0),
        ('K', 0),
        ('Q', 0),
        ('T', 0),
        ('9', 0),
        ('8', 0),
        ('7', 0),
        ('6', 0),
        ('5', 0),
        ('4', 0),
        ('3', 0),
        ('2', 0),
        ('J', 0),
    ]);

    for card in cards.chars() {
        histogram.insert(
            card,
            histogram
                .get(&card)
                .expect(format!("Invalid card value found: {}", card).as_str())
                + 1,
        );
    }

    histogram
}

fn score_helper(counts: Vec<u8>) -> i64 {
    if counts.contains(&5) {
        return 6;
    }
    if counts.contains(&4) {
        return 5;
    }
    if counts.contains(&3) {
        if counts.contains(&2) {
            return 4;
        }
        return 3;
    }

    let two_count: usize = counts
        .into_iter()
        .filter(|value| *value == 2)
        .collect::<Vec<u8>>()
        .len();
    match two_count {
        2 => 2,
        1 => 1,
        _ => 0,
    }
}

fn part_one_hand_score(cards: &String) -> i64 {
    let histogram = get_hand_histogram(cards);
    let counts = histogram.values().cloned().collect();
    score_helper(counts)
}

fn part_two_hand_score(cards: &String) -> i64 {
    let mut histogram = get_hand_histogram(cards);
    let wild_count = histogram.get(&'J').unwrap().clone();
    histogram.insert('J', 0);

    let max_key = histogram
        .keys()
        .into_iter()
        .reduce(|a, b| {
            if histogram.get(a).unwrap() > histogram.get(b).unwrap() {
                a
            } else {
                b
            }
        })
        .unwrap();
    histogram.insert(*max_key, histogram.get(max_key).unwrap() + wild_count);

    let counts = histogram.values().cloned().collect();
    score_helper(counts)
}

lazy_static! {
    static ref PART_ONE_HIGH_CARD: HashMap<char, i8> = HashMap::from([
        ('A', 12),
        ('K', 11),
        ('Q', 10),
        ('J', 9),
        ('T', 8),
        ('9', 7),
        ('8', 6),
        ('7', 5),
        ('6', 4),
        ('5', 3),
        ('4', 2),
        ('3', 1),
        ('2', 0),
    ]);
}

lazy_static! {
    static ref PART_TWO_HIGH_CARD: HashMap<char, i8> = HashMap::from([
        ('A', 12),
        ('K', 11),
        ('Q', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
        ('J', 0),
    ]);
}

fn compare_cards(
    a: &String,
    b: &String,
    hand_score: fn(&String) -> i64,
    high_card: &HashMap<char, i8>,
) -> Ordering {
    let difference = hand_score(a) - hand_score(b);
    if difference > 0 {
        return Ordering::Greater;
    }
    if difference < 0 {
        return Ordering::Less;
    }
    for i in 0..a.len() {
        let difference = high_card.get(&a.chars().nth(i).unwrap()).unwrap()
            - high_card.get(&b.chars().nth(i).unwrap()).unwrap();
        if difference > 0 {
            return Ordering::Greater;
        }
        if difference < 0 {
            return Ordering::Less;
        }
    }

    Ordering::Equal
}

fn part_one_compare_cards(a: &String, b: &String) -> Ordering {
    compare_cards(a, b, part_one_hand_score, &PART_ONE_HIGH_CARD)
}

fn part_two_compare_cards(a: &String, b: &String) -> Ordering {
    compare_cards(a, b, part_two_hand_score, &PART_TWO_HIGH_CARD)
}

fn get_sum(hands: &mut Vec<Hand>, comparison: fn(&String, &String) -> Ordering) -> u64 {
    hands.sort_by(|a, b| comparison(&a.cards, &b.cards));
    let mut sum = 0;
    for i in 0..hands.len() {
        let rank = i as u64 + 1;
        sum += rank * hands[i].bid;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hands() {
        let lines = vec![String::from("32T3K 765"), String::from("KK677 28")];
        let hands = get_hands(lines);

        assert_eq!(hands.len(), 2);
        assert_eq!(hands[0].cards, "32T3K");
        assert_eq!(hands[0].bid, 765);
        assert_eq!(hands[1].cards, "KK677");
        assert_eq!(hands[1].bid, 28);
    }

    #[test]
    fn test_get_hand_histogram() {
        let cards = String::from("AKQT98765432J");
        let histogram = get_hand_histogram(&cards);

        for key in histogram.keys() {
            assert_eq!(histogram.get(key), Some(&1));
        }
    }

    #[test]
    #[should_panic(expected = "Invalid card value found: f")]
    fn test_get_hand_histogram_invalid_card() {
        let cards = String::from("f");
        let histogram = get_hand_histogram(&cards);
    }

    #[test]
    fn test_score_helper() {
        let six_score_counts = vec![0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0];
        let five_score_counts = vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0];
        let four_score_counts = vec![0, 0, 2, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0];
        let three_score_counts = vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 3];
        let two_score_counts = vec![1, 0, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0];
        let one_score_counts = vec![0, 0, 1, 0, 0, 0, 0, 2, 0, 1, 1, 0, 0];
        let zero_score_counts = vec![0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0];

        assert_eq!(score_helper(six_score_counts), 6);
        assert_eq!(score_helper(five_score_counts), 5);
        assert_eq!(score_helper(four_score_counts), 4);
        assert_eq!(score_helper(three_score_counts), 3);
        assert_eq!(score_helper(two_score_counts), 2);
        assert_eq!(score_helper(one_score_counts), 1);
        assert_eq!(score_helper(zero_score_counts), 0);
    }

    #[test]
    fn test_part_one_hand_score() {
        assert_eq!(part_one_hand_score(&String::from("AAAAA")), 6);
        assert_eq!(part_one_hand_score(&String::from("AA8AA")), 5);
        assert_eq!(part_one_hand_score(&String::from("23332")), 4);
        assert_eq!(part_one_hand_score(&String::from("TTT98")), 3);
        assert_eq!(part_one_hand_score(&String::from("23432")), 2);
        assert_eq!(part_one_hand_score(&String::from("A23A4")), 1);
        assert_eq!(part_one_hand_score(&String::from("23456")), 0);
    }

    #[test]
    fn test_part_two_hand_score() {
        assert_eq!(part_two_hand_score(&String::from("JJJAA")), 6);
        assert_eq!(part_two_hand_score(&String::from("JJ8AA")), 5);
        assert_eq!(part_two_hand_score(&String::from("233J2")), 4);
        assert_eq!(part_two_hand_score(&String::from("TTJ98")), 3);
        assert_eq!(part_two_hand_score(&String::from("23432")), 2);
        assert_eq!(part_two_hand_score(&String::from("J23A4")), 1);
        assert_eq!(part_two_hand_score(&String::from("23456")), 0);
    }

    #[test]
    fn test_part_one_compare_cards() {
        assert_eq!(
            part_one_compare_cards(&String::from("T55J5"), &String::from("KTJJT")),
            Ordering::Greater
        );
        assert_eq!(
            part_one_compare_cards(&String::from("32T3K"), &String::from("KK677")),
            Ordering::Less
        );
        assert_eq!(
            part_one_compare_cards(&String::from("QQQJA"), &String::from("T55J5")),
            Ordering::Greater
        );
        assert_eq!(
            part_one_compare_cards(&String::from("KTJJT"), &String::from("KK677")),
            Ordering::Less
        );
        assert_eq!(
            part_one_compare_cards(&String::from("KTJJT"), &String::from("KTJJT")),
            Ordering::Equal
        );
    }

    #[test]
    fn test_part_two_compare_cards() {
        assert_eq!(
            part_two_compare_cards(&String::from("KTJJT"), &String::from("KK677")),
            Ordering::Greater
        );
        assert_eq!(
            part_two_compare_cards(&String::from("32T3K"), &String::from("KK677")),
            Ordering::Less
        );
        assert_eq!(
            part_two_compare_cards(&String::from("KTJJT"), &String::from("QQQJA")),
            Ordering::Greater
        );
        assert_eq!(
            part_two_compare_cards(&String::from("T55J5"), &String::from("QQQJA")),
            Ordering::Less
        );
        assert_eq!(
            part_two_compare_cards(&String::from("KTJJT"), &String::from("KTJJT")),
            Ordering::Equal
        );
    }
}
