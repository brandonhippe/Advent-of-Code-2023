use relative_path::RelativePath;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut sum: i64 = 0;
    for line in contents.lines() {
        let nums: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        sum += extrapolate(nums, true);
    }

    return sum;
}

fn part2(contents: String) -> i64 {
    let mut sum: i64 = 0;
    for line in contents.lines() {
        let nums: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        sum += extrapolate(nums, false);
    }

    return sum;
}

fn extrapolate(nums: Vec<i64>, end: bool) -> i64 {

    if HashSet::<i64>::from_iter(nums.clone()).len() == 1 {
        return nums[0];
    }

    let mut new_nums: Vec<i64> = Vec::new();

    for i in 1..nums.len() {
        new_nums.push(nums[i] - nums[i - 1]);
    }

    return if end {
        nums.last().unwrap() + extrapolate(new_nums, end)
    } else {
        nums[0] - extrapolate(new_nums, end)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 114);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 2);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "9".to_string();
	
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
        "\nPart 1:\nSum of extrapolated values: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSum of extrapolated values: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}