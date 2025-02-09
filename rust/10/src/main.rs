use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let mut connections: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '|' => connections.insert(
                    (x as i32, y as i32),
                    vec![(x as i32, y as i32 + 1), (x as i32, y as i32 - 1)],
                ),
                '-' => connections.insert(
                    (x as i32, y as i32),
                    vec![(x as i32 + 1, y as i32), (x as i32 - 1, y as i32)],
                ),
                'L' => connections.insert(
                    (x as i32, y as i32),
                    vec![(x as i32 + 1, y as i32), (x as i32, y as i32 - 1)],
                ),
                'J' => connections.insert(
                    (x as i32, y as i32),
                    vec![(x as i32 - 1, y as i32), (x as i32, y as i32 - 1)],
                ),
                '7' => connections.insert(
                    (x as i32, y as i32),
                    vec![(x as i32 - 1, y as i32), (x as i32, y as i32 + 1)],
                ),
                'F' => connections.insert(
                    (x as i32, y as i32),
                    vec![(x as i32 + 1, y as i32), (x as i32, y as i32 + 1)],

                ),
                'S' => connections.insert((x as i32, y as i32), vec![]),
                _ => vec![(x as i32, y as i32)].into(),
            };
        }
    }

    let start = connections.iter().find(|(_, v)| v.len() == 0).unwrap().0;
    let mut node: (i32, i32) = (-1, -1);
    for y in -1..=1 {
        for x in -1..=1 {
            let curr_node = (start.0 + x, start.1 + y);
            if connections.contains_key(&curr_node)
                && connections.get(&curr_node).unwrap().contains(&start)
            {
                node = curr_node;
            }
        }
    }

    let mut count = 1;
    let mut prev_node = *start;
    while node != *start {
        let mut next_node = (0, 0);
        for n in connections.get(&node).unwrap() {
            if *n != prev_node {
                next_node = *n;
            }
        }
        prev_node = node;
        node = next_node;
        count += 1;
    }

    return count / 2;
}

fn part2(contents: String) -> i32 {
    let mut connections: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '|' => connections.insert(
                    (x as i32, y as i32),
                    vec![(x as i32, y as i32 + 1), (x as i32, y as i32 - 1)],
                ),
                '-' => connections.insert(
                    (x as i32, y as i32),
                    vec![(x as i32 + 1, y as i32), (x as i32 - 1, y as i32)],
                ),
                'L' => connections.insert(
                    (x as i32, y as i32),
                    vec![(x as i32 + 1, y as i32), (x as i32, y as i32 - 1)],
                ),
                'J' => connections.insert(
                    (x as i32, y as i32),
                    vec![(x as i32 - 1, y as i32), (x as i32, y as i32 - 1)],
                ),
                '7' => connections.insert(
                    (x as i32, y as i32),
                    vec![(x as i32 - 1, y as i32), (x as i32, y as i32 + 1)],
                ),
                'F' => connections.insert(
                    (x as i32, y as i32),
                    vec![(x as i32 + 1, y as i32), (x as i32, y as i32 + 1)],
                ),
                'S' => connections.insert((x as i32, y as i32), vec![]),
                _ => vec![(x as i32, y as i32)].into(),
            };
        }
    }

    let start = connections.iter().find(|(_, v)| v.len() == 0).unwrap().0;
    let mut starting_node: (i32, i32) = (-1, -1);
    for y in -1..=1 {
        for x in -1..=1 {
            let curr_node = (start.0 + x, start.1 + y);
            if connections.contains_key(&curr_node)
                && connections.get(&curr_node).unwrap().contains(&start)
            {
                starting_node = curr_node;
            }
        }
    }

    let mut loop_nodes: HashSet<(i32, i32)> = HashSet::new();
    loop_nodes.insert(*start);
    let mut node: (i32, i32) = starting_node;
    let mut prev_node = *start;
    while node != *start {
        loop_nodes.insert(node);
        let mut next_node = (0, 0);
        for n in connections.get(&node).unwrap() {
            if *n != prev_node {
                next_node = *n;
            }
        }
        prev_node = node;
        node = next_node;
    }

    connections.insert(*start, vec![starting_node, prev_node]);

    let mut inside_count: i32 = 0;
    for y in 1..contents.lines().count() - 1 {
        let mut inside: bool = false;
        let mut side_set: HashSet<i32> = HashSet::from_iter(vec![y as i32]);

        for x in 0..contents.lines().next().unwrap().chars().count() {
            let node = (x as i32, y as i32);

            if loop_nodes.contains(&node) {
                let neighbor_set: HashSet<i32> =
                    HashSet::from_iter(connections.get(&node).unwrap().iter().map(|n| n.1));
                side_set = side_set.union(&neighbor_set).cloned().collect();

                if !connections
                    .get(&node)
                    .unwrap()
                    .contains(&(x as i32 + 1, y as i32))
                {
                    if side_set.len() == 3 {
                        inside = !inside;
                    }

                    side_set = HashSet::from_iter(vec![y as i32]);
                }
            } else {
                inside_count += inside as i32;
            }
        }
    }

    return inside_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("p1_example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 8);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("p2_example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 10);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "10".to_string();
	
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
        "\nPart 1:\nSteps to furthest point on loop: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nTiles enclosed by loop: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}