use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    return contents
        .lines()
        .nth(0)
        .unwrap()
        .split(",")
        .map(|line| hashmap(line.to_string()))
        .sum();
}

fn part2(contents: String) -> i32 {
    let mut boxes: Vec<Vec<(String, i32)>> = Vec::from(vec![Vec::new(); 256]);

    for lens in contents.lines().nth(0).unwrap().split(",") {
        if lens.contains('-') {
            let hash: i32 = hashmap(lens[..lens.find('-').unwrap()].to_string());
            let ix: usize = boxes[hash as usize]
                .iter()
                .position(|x| x.0 == lens[..lens.find('-').unwrap()].to_string())
                .unwrap_or(usize::MAX);

            if ix != usize::MAX {
                boxes[hash as usize].remove(ix);
            }
        } else {
            let hash: i32 = hashmap(lens[..lens.find('=').unwrap()].to_string());
            let ix: usize = boxes[hash as usize]
                .iter()
                .position(|x| x.0 == lens[..lens.find('=').unwrap()].to_string())
                .unwrap_or(usize::MAX);

            let f_len: i32 = lens[lens.find('=').unwrap() + 1..].parse().unwrap();

            if ix == usize::MAX {
                boxes[hash as usize].push((lens[..lens.find('=').unwrap()].to_string(), f_len));
            } else {
                boxes[hash as usize][ix].1 = f_len;
            }
        }
    }

    return boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(j, x)| (j + 1) as i32 * x.1)
                .sum::<i32>()
                * (i + 1) as i32
        })
        .sum();
}

fn hashmap(s: String) -> i32 {
    let mut hash_sum: i32 = 0;
    for c in s.chars() {
        hash_sum += c as i32;
        hash_sum *= 17;
        hash_sum %= 256;
    }

    return hash_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 1320);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 145)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "15".to_string();
	
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
        "\nPart 1:\nSum of hash algorithms: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nFocusing power: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}