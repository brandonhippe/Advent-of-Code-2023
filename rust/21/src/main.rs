use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

fn part1(contents: String, steps: i64) -> i64 {
    let mut area: HashSet<(i64, i64)> = HashSet::new();
    let mut start: (i64, i64) = (0, 0);
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                continue;
            }

            area.insert((x as i64, y as i64));
            if c == 'S' {
                start = (x as i64, y as i64);
            }
        }
    }

    let dim: i64 = contents.lines().count() as i64;
    return bfs_dists(area.clone(), start, steps, dim).values().filter(|&&d| d <= steps && d % 2 == steps % 2).count() as i64;
}

fn part2(contents: String, steps: i64) -> i64 {
    let mut area: HashSet<(i64, i64)> = HashSet::new();
    let mut start: (i64, i64) = (0, 0);
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                continue;

            }

            area.insert((x as i64, y as i64));
            if c == 'S' {
                start = (x as i64, y as i64);
            }
        }
    }

    let dim: i64 = contents.lines().count() as i64;
    let dists: HashMap<(i64, i64), i64> = bfs_dists(area.clone(), start, dim * 4 + start.0, dim);

    let counts: Vec<i64> = Vec::from_iter((0..3).map(|i| dists.values().filter(|&&d| d <= dim * 2 * i + start.0 && d % 2 == steps % 2).count() as i64));

    let a = (counts[2] - 2 * counts[1] + counts[0]) / 2;
    let b = counts[1] - counts[0] - a;
    let c = counts[0];
    let n = steps / (2 * dim);

    return a * n * n + b * n + c;
}

fn bfs_dists(area: HashSet<(i64, i64)>, start: (i64, i64), max_steps: i64, dim: i64) -> HashMap<(i64, i64), i64> {
    let mut open: VecDeque<((i64, i64), i64)> = VecDeque::from(vec![(start, 0)]);
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut dists: HashMap<(i64, i64), i64> = HashMap::new();

    while let Some((pos, dist)) = open.pop_front() {
        if dist > max_steps || visited.contains(&pos) {
            continue;
        }

        visited.insert(pos);

        if (max_steps - dist) % 2 == 0 {
            dists.insert(pos, dist);
        }

        for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_x = pos.0 + dx;
            let new_y = pos.1 + dy;
            if area.contains(&(((new_x % dim) + dim) % dim, ((new_y % dim) + dim) % dim)) {
                open.push_back(((new_x, new_y), dist + 1));
            }
        }
    }

    return dists;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents, 6), 16);

    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "21".to_string();
	
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
        "\nPart 1:\nTiles reached in 64 steps: {}\nRan in {:.5?}",
        part1(contents.clone(), 64),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nTiles reached in 26501365 steps: {}\nRan in {:.5?}",
        part2(contents.clone(), 26501365),
        part2_timer.elapsed()
    );
}