use cached::proc_macro::cached;
use relative_path::RelativePath;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    return contents
        .lines()
        .map(|line| {
            let spring_line = line.split(" ").nth(0).unwrap().to_string();
            let counts = line
                .split(" ")
                .nth(1)
                .unwrap()
                .to_string()
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<VecDeque<i32>>();
            spring_combos(spring_line, counts)
        })
        .sum();
}

fn part2(contents: String) -> i64 {
    return contents
        .lines()
        .map(|line| {
            let spring_line = vec![line.split(" ").nth(0).unwrap().to_string(); 5].join("?");
            let counts = vec![line.split(" ").nth(1).unwrap().to_string(); 5]
                .join(",")
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())

                .collect::<VecDeque<i32>>();
            spring_combos(spring_line, counts)
        })
        .sum();
}

#[cached]
fn spring_combos(spring_line: String, counts: VecDeque<i32>) -> i64 {
    if counts.len() == 0 {
        let expect_set: HashSet<char> = HashSet::from_iter(vec!['.']);
        let spring_set = HashSet::from_iter(spring_line.chars());
        return (expect_set.union(&spring_set).collect::<Vec<&char>>().len() == 1) as i64;
    }

    let mut counts_mut = &mut counts.clone();
    let mut goal = counts_mut.pop_front().unwrap();
    let mut count = 0;

    let mut good_to = 0;
    for (i, c) in spring_line.chars().enumerate() {
        if count > goal {
            return 0;
        }

        match c {
            '#' => count += 1,
            '.' => {
                if count != 0 && count != goal {
                    return 0;
                }

                good_to = i + 1;
                if count != 0 {
                    count = 0;
                    goal = counts_mut.pop_front().unwrap_or(0);
                }
            },
            '?' => {
                let mut combs: i64 = 0;
                counts_mut.push_front(goal);

                if count == 0 || count == goal {
                    let empty_line = spring_line
                        .chars()
                        .enumerate()
                        .skip(good_to)
                        .map(|(j, c)| if j == i { '.' } else { c })
                        .collect::<String>();

                    combs += spring_combos(empty_line, counts_mut.clone());
                }

                if count != goal {
                    let filled_line = spring_line
                        .chars()
                        .enumerate()
                        .skip(good_to)
                        .map(|(j, c)| if j == i { '#' } else { c })
                        .collect::<String>();

                    combs += spring_combos(filled_line, counts_mut.clone());
                }

                return combs;
            },
            _ => panic!("Invalid char"),
        }
    }

    return (count == goal && counts_mut.len() == 0) as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 21);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents.clone()), 525152)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "12".to_string();
	
	let root = env::current_dir().unwrap();
	let path_str = if args.len() > 1 {
	    args[1].clone()
    } else if root.ends_with(format!("{}", day)) {
	    format!("../../../Inputs/{}_{}.txt", year, day)
	} else {
	    format!("/Inputs/{}_{}.txt", year, day)
	};

    let contents = fs::read_to_string(if args.len() > 1 {path_str} else {RelativePath::new(&path_str).to_path(&root).display().to_string()})
        .expect("Should have been able to read the file");

    let part1_timer = Instant::now();
    println!(
        "\nPart 1:\nSum of possible arrangements: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSum of possible arrangements: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}