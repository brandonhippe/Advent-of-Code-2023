use regex::Regex;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

struct Game {
    id: i32,
    cubes: String,
}

fn part1(contents: String) -> i32 {
    let maxCubes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let re_game = Regex::new(r"Game (\d+): (.*)").unwrap();
    let re_cubes = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    let mut id_sum = 0;
    for line in contents.lines() {
        let game: Game = re_game
            .captures(line)
            .map(|caps| {
                let (_, [id, cubes]) = caps.extract();
                Game {
                    id: id.parse::<i32>().unwrap(),
                    cubes: cubes.to_string(),
                }
            })
            .unwrap();

        let cubes: Vec<(&str, i32)> = re_cubes
            .captures_iter(&game.cubes)
            .map(|caps| {
                let (_, [num, color]) = caps.extract();

                (color, num.parse::<i32>().unwrap())
            })
            .collect();

        let mut possible: bool = true;
        for (color, num) in cubes {
            if num > maxCubes[color] {
                possible = false;
                break;
            }
        }

        if possible {
            id_sum += game.id;
        }
    }

    return id_sum;
}

fn part2(contents: String) -> i32 {
    let re_game = Regex::new(r"Game (\d+): (.*)").unwrap();
    let re_cubes = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    let mut power_sum = 0;
    for line in contents.lines() {
        let game: Game = re_game
            .captures(line)
            .map(|caps| {
                let (_, [id, cubes]) = caps.extract();
                Game {
                    id: id.parse::<i32>().unwrap(),
                    cubes: cubes.to_string(),
                }
            })
            .unwrap();

        let cubes: Vec<(&str, i32)> = re_cubes
            .captures_iter(&game.cubes)
            .map(|caps| {
                let (_, [num, color]) = caps.extract();
                (color, num.parse::<i32>().unwrap())
            })
            .collect();

        let mut maxCubes = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        for (color, num) in cubes {
            if num > maxCubes[color] {
                maxCubes.insert(color, num);
            }
        }

        let mut power: i32 = 1;
        for num in maxCubes.values() {
            power *= num;
        }

        power_sum += power;
    }

    return power_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 8);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 2286);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "2".to_string();
	
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
        "\nPart 1:\nSum of ID's of possible games: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSum of power of games: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}