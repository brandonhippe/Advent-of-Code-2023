use regex::Regex;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    let re_num = Regex::new(r"(\d+)").unwrap();
    let mut nums: Vec<(i64, i64)> = re_num
        .captures_iter(&lines[0])
        .map(|caps| {
            let (_, [num]) = caps.extract();
            (num.parse::<i64>().unwrap(), num.parse::<i64>().unwrap())
        })
        .collect();

    let re_conv = Regex::new(r"^(\d+)\s+(\d+)\s+(\d+)$").unwrap();
    let mut conversions: Vec<(i64, i64, i64)> = Vec::new();

    for line in lines[2..lines.len()].iter() {
        if line.len() == 0 {
            continue;
        }

        let temp_conv: Option<(i64, i64, i64)> = re_conv.captures(line).map(|caps| {
            let (_, [t, f, d]) = caps.extract();
            (
                t.parse::<i64>().unwrap(),
                f.parse::<i64>().unwrap(),
                d.parse::<i64>().unwrap(),
            )
        });


        if temp_conv.is_some() {
            // Add conversion
            conversions.push(temp_conv.unwrap());
        } else {
            // Convert the current nums
            let mut new_nums = applyConversions(&conversions, &mut nums);
            conversions.clear();
            nums.append(&mut new_nums);
        }
    }

    let mut new_nums = applyConversions(&conversions, &mut nums);
    nums.append(&mut new_nums);

    let mut minimum = nums[0].0;
    for (start, _) in nums[1..nums.len()].iter() {
        if *start < minimum {
            minimum = *start;
        }
    }

    return minimum;
}

fn part2(contents: String) -> i64 {
    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    let re_num = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    let mut nums: Vec<(i64, i64)> = re_num
        .captures_iter(&lines[0])
        .map(|caps| {
            let (_, [s, e]) = caps.extract();
            (
                s.parse::<i64>().unwrap(),
                s.parse::<i64>().unwrap() + e.parse::<i64>().unwrap() - 1,
            )
        })
        .collect();

    let re_conv = Regex::new(r"^(\d+)\s+(\d+)\s+(\d+)$").unwrap();
    let mut conversions: Vec<(i64, i64, i64)> = Vec::new();

    for line in lines[2..lines.len()].iter() {
        if line.len() == 0 {
            continue;
        }

        let temp_conv: Option<(i64, i64, i64)> = re_conv.captures(line).map(|caps| {
            let (_, [t, f, d]) = caps.extract();
            (
                t.parse::<i64>().unwrap(),
                f.parse::<i64>().unwrap(),
                d.parse::<i64>().unwrap(),
            )
        });

        if temp_conv.is_some() {
            // Add conversion
            conversions.push(temp_conv.unwrap());
        } else {
            // Convert the current nums
            let mut new_nums = applyConversions(&conversions, &mut nums);
            conversions.clear();
            nums.append(&mut new_nums);
        }
    }

    let mut new_nums = applyConversions(&conversions, &mut nums);
    nums.append(&mut new_nums);

    let mut minimum = nums[0].0;
    for (start, _) in nums[1..nums.len()].iter() {
        if *start < minimum {
            minimum = *start;
        }
    }

    return minimum;
}

fn applyConversions(
    conversions: &Vec<(i64, i64, i64)>,
    nums: &mut Vec<(i64, i64)>,
) -> Vec<(i64, i64)> {
    let mut new_nums: Vec<(i64, i64)> = Vec::new();
    while nums.len() != 0 {
        for i in (0..nums.len()).rev() {
            let (start, end) = nums[i];
            let mut split = false;

            for (dest_start, src_start, d) in conversions {
                if *src_start <= start && end < *src_start + *d {
                    new_nums.push((
                        convert(*dest_start, *src_start, start),
                        convert(*dest_start, *src_start, end),
                    ));
                    nums.remove(i);
                    split = true;
                    break;
                } else if *src_start <= start && start < *src_start + *d {
                    nums[i] = (start, *src_start + *d - 1);
                    nums.push((*src_start + *d, end));
                    split = true;
                    break;
                } else if *src_start <= end && end < *src_start + *d {
                    nums[i] = (*src_start, end);
                    nums.push((start, *src_start - 1));
                    split = true;
                    break;
                }
            }

            if !split {
                new_nums.push((start, end));
                nums.remove(i);
            }
        }
    }

    return new_nums;
}

fn convert(dest_start: i64, from_start: i64, num: i64) -> i64 {
    return dest_start + (num - from_start);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 35);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 46);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "5".to_string();
	
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
        "\nPart 1:\nLowest location number: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nLowest location number: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}