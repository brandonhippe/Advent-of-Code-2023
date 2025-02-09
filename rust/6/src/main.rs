use itertools::Itertools;
use regex::Regex;
use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let re_nums = Regex::new(r"(\d+)").unwrap();

    let (times, dists): (Vec<i64>, Vec<i64>) = contents
        .lines()
        .map(|line| {
            let nums: Vec<i64> = re_nums
                .captures_iter(line)
                .map(|caps| {
                    let (_, [num]) = caps.extract();
                    num.parse::<i64>().unwrap()
                })
                .collect();

            nums
        })
        .collect_tuple()
        .unwrap();

    let mut product = 1;
    for (time, dist) in times.iter().zip(dists.iter()) {
        product *= beat_record(*dist as f64, *time as f64);
    }

    return product;
}

fn part2(contents: String) -> i64 {

    let re_nums = Regex::new(r"(\d+)").unwrap();

    let (time, dist): (i64, i64) = contents
        .replace(" ", "")
        .lines()
        .map(|line| {
            let nums: i64 = re_nums
                .captures(line)
                .map(|caps| {
                    let (_, [num]) = caps.extract();
                    num.parse::<i64>().unwrap()
                })
                .unwrap();

            nums
        })
        .collect_tuple()
        .unwrap();

    return beat_record(dist as f64, time as f64);
}

fn beat_record(dist: f64, time: f64) -> i64 {
    let mut smaller: f64 = (time - (time.powf(2.0) - (4.0 * dist)).sqrt()) / 2.0;
    let mut larger: f64 = (time + (time.powf(2.0) - (4.0 * dist)).sqrt()) / 2.0;

    if smaller.ceil() == smaller {
        smaller += 1.0
    }

    if larger.floor() == larger {
        larger -= 1.0
    }

    return larger.floor() as i64 - smaller.ceil() as i64 + 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 288);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 71503);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "6".to_string();
	
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
        "\nPart 1:\nProduct of ways to beat records: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nWays to beat record: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}