use itertools::{Itertools, MultiProduct};
use regex::Regex;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let card_vals: HashMap<char, i32> = HashMap::from_iter([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]);
    let hand_scores: Vec<Vec<i32>> = Vec::from([
        Vec::from([1, 1, 1, 1, 1]),
        Vec::from([1, 1, 1, 2]),
        Vec::from([1, 2, 2]),
        Vec::from([1, 1, 3]),
        Vec::from([2, 3]),
        Vec::from([1, 4]),
        Vec::from([5]),
    ]);


    let hand_regex = Regex::new(r"(.{5}) (\d+)").unwrap();
    let mut hands: HashMap<String, i32> = HashMap::from_iter(contents.lines().map(|line| {
        let hand: (String, i32) = hand_regex
            .captures(line)
            .map(|caps| {
                let (_, [hand, num]) = caps.extract();
                (hand.to_string(), num.parse::<i32>().unwrap())
            })
            .unwrap();

        hand
    }));

    let mut sort_vec: Vec<String> = Vec::from_iter(hands.keys().map(|key| key.to_string()));
    sort_vec.sort_by_cached_key(|hand| {
        let card_set: HashSet<char> = HashSet::from_iter(hand.chars());
        let mut card_counts: Vec<i32> = Vec::from_iter(
            card_set
                .iter()
                .map(|card| hand.matches(*card).count() as i32),
        );
        card_counts.sort();

        let mut hand_score: i64 = hand_scores
            .iter()
            .position(|count| count == &card_counts)
            .unwrap() as i64
            + 1;
        for c in hand.chars() {
            hand_score = hand_score * 100 + card_vals[&c] as i64;
        }

        hand_score
    });

    return sort_vec
        .iter()
        .enumerate()
        .map(|(i, hand)| hands[hand] * (i as i32 + 1))
        .sum();
}

fn part2(contents: String) -> i32 {
    let card_vals: HashMap<char, i32> = HashMap::from_iter([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 1),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]);
    let hand_scores: Vec<Vec<i32>> = Vec::from([
        Vec::from([1, 1, 1, 1, 1]),
        Vec::from([1, 1, 1, 2]),
        Vec::from([1, 2, 2]),
        Vec::from([1, 1, 3]),
        Vec::from([2, 3]),
        Vec::from([1, 4]),
        Vec::from([5]),
    ]);

    let hand_regex = Regex::new(r"(.{5}) (\d+)").unwrap();
    let mut hands: HashMap<String, i32> = HashMap::from_iter(contents.lines().map(|line| {
        let hand: (String, i32) = hand_regex
            .captures(line)
            .map(|caps| {
                let (_, [hand, num]) = caps.extract();
                (hand.to_string(), num.parse::<i32>().unwrap())
            })
            .unwrap();

        hand
    }));

    let mut sort_vec: Vec<String> = Vec::from_iter(hands.keys().map(|key| key.to_string()));
    sort_vec.sort_by_cached_key(|hand| {
        let card_set: HashSet<char> = HashSet::from_iter(hand.chars());
        let mut card_counts: Vec<i32> = Vec::from_iter(card_set.iter().map(|card| {
            if card == &'J' {
                0
            } else {
                hand.matches(*card).count() as i32
            }
        }));
        card_counts.sort();

        while card_counts.len() > 0 && card_counts[0] == 0 {
            card_counts.remove(0);
        }

        let mut hand_score: i64 = 0;
        if card_counts.len() == 0 {
            hand_score = 7;
        } else {
            let non_joker_count: i32 = card_counts.iter().sum();

            if non_joker_count == 5 {
                hand_score = hand_scores
                    .iter()
                    .position(|count| count == &card_counts)
                    .unwrap() as i64
                    + 1;
            }

            for p in (0..card_counts.len()).product_repeat(5 - non_joker_count as usize) {
                let mut new_card_counts = card_counts.clone();
                for i in p {
                    new_card_counts[i] += 1;
                }

                new_card_counts.sort();
                hand_score = std::cmp::max(
                    hand_score,
                    hand_scores
                        .iter()
                        .position(|count| count == &new_card_counts)
                        .unwrap() as i64
                        + 1,
                );
            }
        }

        for c in hand.chars() {
            hand_score = hand_score * 100 + card_vals[&c] as i64;
        }

        hand_score
    });

    return sort_vec
        .iter()
        .enumerate()
        .map(|(i, hand)| hands[hand] * (i as i32 + 1))
        .sum();
}

pub fn product_repeat<I>(it: I, repeat: usize) -> MultiProduct<I>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    std::iter::repeat(it).take(repeat).multi_cartesian_product()
}

pub trait ProductRepeat: Iterator + Clone
where
    Self::Item: Clone,
{
    fn product_repeat(self, repeat: usize) -> MultiProduct<Self> {
        std::iter::repeat(self)
            .take(repeat)
            .multi_cartesian_product()
    }
}

impl<T: Iterator + Clone> ProductRepeat for T where T::Item: Clone {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 6440);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 5905);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "7".to_string();
	
	let root = env::current_dir().unwrap();
	let path_str = if args.len() > 1 {
	    args[1].clone()
	} else if root.ends_with(format!("rust_{}_{}", year, day)) {
	    format!("../../../Inputs/{}_{}.txt", year, day)
	} else {
	    format!("/Inputs/{}_{}.txt", year, day)
	};

    let contents = fs::read_to_string(if args.len() > 1 {path_str} else {RelativePath::new(&path_str).to_path(&root).display().to_string()})
        .expect("Should have been able to read the file");

    let part1_timer = Instant::now();
    println!(
        "\nPart 1:\nTotal Winnings: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nTotal Winnings: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}