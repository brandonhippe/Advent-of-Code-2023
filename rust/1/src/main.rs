use regex::Regex;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let re_multDig = Regex::new(r"^\D*(\d{1}).*(\d{1})\D*$").unwrap();
    let re_singDig = Regex::new(r"^\D*(\d{1})\D*$").unwrap();

    let mut sum = 0;
    for line in contents.lines() {
        let digits: Vec<(i32, i32)> = re_multDig
            .captures_iter(line)
            .map(|caps| {
                let (_, [d10, d1]) = caps.extract();
                (d10.parse::<i32>().unwrap(), d1.parse::<i32>().unwrap())
            })
            .collect();

        if digits.len() == 1 {
            sum += digits[0].0 * 10 + digits[0].1;
        } else {
            let digit: Vec<i32> = re_singDig
                .captures_iter(line)
                .map(|caps| {
                    let (_, [d]) = caps.extract();
                    d.parse::<i32>().unwrap()
                })
                .collect();

            sum += digit[0] * 11;
        }
    }


    return sum;
}

fn part2(contents: String) -> i32 {
    let mut mapping = HashMap::new();
    let num_words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut ix = 1;
    for word in num_words {
        mapping.insert(word, ix);
        ix += 1;
    }

    let re_multDig = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d{1}).*(one|two|three|four|five|six|seven|eight|nine|\d{1})").unwrap();
    let re_singDig = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d{1})").unwrap();

    let mut sum = 0;
    for line in contents.lines() {
        let digits: Vec<(&str, &str)> = re_multDig
            .captures_iter(line)
            .map(|caps| {
                let (_, [d10, d1]) = caps.extract();
                (d10, d1)
            })
            .collect();

        if digits.len() == 1 {
            if mapping.contains_key(digits[0].0) {
                sum += mapping[digits[0].0] * 10;
            } else {
                sum += digits[0].0.parse::<i32>().unwrap() * 10;
            }

            if mapping.contains_key(digits[0].1) {
                sum += mapping[digits[0].1];
            } else {
                sum += digits[0].1.parse::<i32>().unwrap();
            }
        } else {
            let digit: Vec<&str> = re_singDig
                .captures_iter(line)
                .map(|caps| {
                    let (_, [d]) = caps.extract();
                    d
                })
                .collect();

            if mapping.contains_key(digit[0]) {
                sum += mapping[digit[0]] * 11;
            } else {
                sum += digit[0].parse::<i32>().unwrap() * 11;
            }
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("p1_example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 142);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("p2_example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 281);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "1".to_string();
	
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
        "\nPart 1:\nSum of Calibration Values: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSum of Calibration Values: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}