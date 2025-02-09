use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let schematic = contents.lines().collect::<Vec<&str>>();

    let mut digits = HashMap::new();
    let mut chars = HashSet::new();
    for (y, line) in schematic.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                digits.insert((x as i32, y as i32), c.to_digit(10).unwrap() as i32);
            } else if c != '.' {
                chars.insert((x as i32, y as i32));
            }
        }
    }

    let mut n_sum = 0;
    for (x, y) in chars {
        for y_off in -1..2 as i32 {
            for x_off in -1..2 as i32 {
                if digits.contains_key(&(x + x_off, y + y_off)) {
                    let mut p_x = x + x_off;
                    let p_y = y + y_off;
                    while digits.contains_key(&(p_x - 1, p_y)) {
                        p_x -= 1;
                    }

                    let mut num = 0;
                    while digits.contains_key(&(p_x, p_y)) {

                        num *= 10;
                        num += digits[&(p_x, p_y)];
                        digits.remove(&(p_x, p_y));
                        p_x += 1;
                    }

                    n_sum += num;
                }
            }
        }
    }

    return n_sum;
}

fn part2(contents: String) -> i32 {
    let schematic = contents.lines().collect::<Vec<&str>>();

    let mut digits = HashMap::new();
    let mut gears = HashSet::new();
    for (y, line) in schematic.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                digits.insert((x as i32, y as i32), c.to_digit(10).unwrap() as i32);
            } else if c == '*' {
                gears.insert((x as i32, y as i32));
            }
        }
    }

    let mut ratio_sum = 0;
    for (x, y) in gears {
        let mut neighbors: Vec<i32> = Vec::new();
        for y_off in -1..2 as i32 {
            for x_off in -1..2 as i32 {
                if digits.contains_key(&(x + x_off, y + y_off)) {
                    let mut p_x = x + x_off;
                    let p_y = y + y_off;
                    while digits.contains_key(&(p_x - 1, p_y)) {
                        p_x -= 1;
                    }

                    let mut num = 0;
                    while digits.contains_key(&(p_x, p_y)) {
                        num *= 10;
                        num += digits[&(p_x, p_y)];
                        digits.remove(&(p_x, p_y));
                        p_x += 1;
                    }

                    neighbors.push(num);
                }
            }
        }

        if neighbors.len() == 2 {
            ratio_sum += neighbors[0] * neighbors[1];
        }
    }

    return ratio_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 4361);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 467835);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "3".to_string();
	
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
        "\nPart 1:\nSum of part numbers: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSum of gear ratios: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}