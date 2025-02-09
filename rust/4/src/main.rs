use regex::Regex;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::iter::Sum;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let re_card = regex::Regex::new(r"Card\s+\d+:\s+(.*)\s+[|]\s+(.*)").unwrap();
    let re_nums = regex::Regex::new(r"(\d+)").unwrap();

    let mut points = 0;
    for line in contents.lines() {
        let card: (String, String) = re_card
            .captures(line)
            .map(|caps| {
                let (_, [winning, nums]) = caps.extract();
                (winning.to_string(), nums.to_string())
            })
            .unwrap();

        let winning = HashSet::<_>::from_iter(re_nums.captures_iter(&card.0).map(|caps| {
            let (_, [num]) = caps.extract();
            num.parse::<i32>().unwrap()
        }));

        let nums = HashSet::<_>::from_iter(re_nums.captures_iter(&card.1).map(|caps| {
            let (_, [num]) = caps.extract();
            num.parse::<i32>().unwrap()
        }));

        let wins: HashSet<_> = winning.intersection(&nums).collect();


        if wins.len() != 0 {
            points += 1 << wins.len() - 1;
        }
    }

    return points;
}

struct Card {
    id: i32,
    winning: String,
    nums: String,
}

fn part2(contents: String) -> i32 {
    let re_card = regex::Regex::new(r"Card\s+(\d+):\s+(.*)\s+[|]\s+(.*)").unwrap();
    let re_nums = regex::Regex::new(r"(\d+)").unwrap();

    let mut cards: HashMap<i32, i32> = HashMap::from_iter(contents.lines().map(|line| {
        let id = re_card
            .captures(line)
            .map(|caps| {
                let (_, [id, _, _]) = caps.extract();
                id.parse::<i32>().unwrap()
            })
            .unwrap();
        (id, 1)
    }));

    for line in contents.lines() {
        let card: Card = re_card
            .captures(line)
            .map(|caps| {
                let (_, [id, winning, nums]) = caps.extract();
                Card {
                    id: id.parse::<i32>().unwrap(),
                    winning: winning.to_string(),
                    nums: nums.to_string(),
                }
            })
            .unwrap();

        let winning = HashSet::<_>::from_iter(re_nums.captures_iter(&card.winning).map(|caps| {
            let (_, [num]) = caps.extract();
            num.parse::<i32>().unwrap()
        }));

        let nums = HashSet::<_>::from_iter(re_nums.captures_iter(&card.nums).map(|caps| {
            let (_, [num]) = caps.extract();
            num.parse::<i32>().unwrap()
        }));

        let wins: HashSet<_> = winning.intersection(&nums).collect();
        if wins.len() == 0 {
            continue;
        }

        let mult = cards[&card.id];
        for id in card.id + 1..card.id + 1 + (wins.len() as i32) {
            cards.insert(id, cards[&id] + mult);
        }
    }

    return cards.values().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 13);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 30);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "4".to_string();
	
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
        "\nPart 1:\nPoints: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nTotal Scratchcards: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}