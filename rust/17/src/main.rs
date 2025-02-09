use relative_path::RelativePath;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((x as i32, y as i32), c.to_digit(10).unwrap() as i32);
        }
    }

    let mut visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State {
        position: (0, 0),
        from_dir: (1, 0),
        cost: 0,
    });
    queue.push(State {
        position: (0, 0),
        from_dir: (0, 1),
        cost: 0,
    });

    let end: (i32, i32) = (
        *grid.keys().map(|(x, _)| x).max().unwrap(),
        *grid.keys().map(|(_, y)| y).max().unwrap(),
    );


    while let Some(State {
        position,
        from_dir,
        cost,
    }) = queue.pop()
    {
        if position == end {
            return cost;
        }

        if visited.contains(&(position, from_dir)) {
            continue;
        }
        visited.insert((position, from_dir));

        let new_dirs = vec![(from_dir.1, from_dir.0), (-from_dir.1, -from_dir.0)];
        for dir in &new_dirs {
            let mut acc_cost = 0;
            for dist in 1..=3 {
                let new_pos = (position.0 + dir.0 * dist, position.1 + dir.1 * dist);
                if !grid.contains_key(&new_pos) {
                    break;
                }

                acc_cost += grid[&new_pos];

                queue.push(State {
                    position: new_pos,
                    from_dir: *dir,
                    cost: cost + acc_cost,
                });
            }
        }
    }

    return -1;
}

fn part2(contents: String) -> i32 {
    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((x as i32, y as i32), c.to_digit(10).unwrap() as i32);
        }
    }

    let mut visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State {
        position: (0, 0),
        from_dir: (1, 0),
        cost: 0,
    });
    queue.push(State {
        position: (0, 0),
        from_dir: (0, 1),
        cost: 0,
    });

    let end: (i32, i32) = (
        *grid.keys().map(|(x, _)| x).max().unwrap(),
        *grid.keys().map(|(_, y)| y).max().unwrap(),
    );

    while let Some(State {
        position,
        from_dir,
        cost,
    }) = queue.pop()
    {
        if position == end {
            return cost;
        }

        if visited.contains(&(position, from_dir)) {
            continue;
        }
        visited.insert((position, from_dir));

        let new_dirs = vec![(from_dir.1, from_dir.0), (-from_dir.1, -from_dir.0)];
        for dir in &new_dirs {
            let mut acc_cost = 0;
            for dist in 1..=10 {
                let new_pos = (position.0 + dir.0 * dist, position.1 + dir.1 * dist);
                if !grid.contains_key(&new_pos) {
                    break;
                }

                acc_cost += grid[&new_pos];

                if dist < 4 {
                    continue;
                }

                queue.push(State {
                    position: new_pos,
                    from_dir: *dir,
                    cost: cost + acc_cost,
                });
            }
        }
    }

    return -1;
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    position: (i32, i32),
    from_dir: (i32, i32),
    cost: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 102);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 94)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "17".to_string();
	
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
        "\nPart 1:\nMinimum heat loss: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nMinimum heat loss: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}