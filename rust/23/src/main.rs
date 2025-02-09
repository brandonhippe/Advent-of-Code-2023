use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut slopes: HashMap<(i64, i64), (i64, i64)> = HashMap::new();
    let mut area: HashSet<(i64, i64)> = HashSet::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => area.insert((x as i64, y as i64)),
                'v' => slopes.insert((x as i64, y as i64), (0, 1)).is_some(),
                '^' => slopes.insert((x as i64, y as i64), (0, -1)).is_some(),
                '<' => slopes.insert((x as i64, y as i64), (-1, 0)).is_some(),
                '>' => slopes.insert((x as i64, y as i64), (1, 0)).is_some(),
                _ => true,
            };
        }
    }

    let mut positions: HashSet<(i64, i64)> = slopes.keys().map(|(x, y)| (*x, *y)).collect();
    positions.extend(area.iter().map(|(x, y)| (*x, *y)));

    let mut nodes: HashMap<(i64, i64), HashMap<(i64, i64), i64>> = HashMap::new();
    for (x, y) in positions.iter() {
        if *y == 0 || *y == contents.lines().count() as i64 - 1 {
            nodes.insert((*x, *y), HashMap::new());
            continue;
        }


        let mut neighbors: Vec<(i64, i64)> = Vec::new();
        for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if positions.contains(&(x + dx, y + dy)) {
                neighbors.push((x + dx, y + dy));
            }
        }

        if neighbors.len() > 2 {
            nodes.insert((*x, *y), HashMap::new());
            continue;
        }
    }

    for start in nodes.clone().keys() {
        let mut queue: VecDeque<((i64, i64), i64)> = VecDeque::new();
        let mut visited: HashSet<(i64, i64)> = HashSet::new();
        queue.push_back((*start, 0));

        while let Some((current, path_len)) = queue.pop_front() {
            if visited.contains(&current) {
                continue;
            }

            visited.insert(current);

            if current != *start && nodes.contains_key(&current) {
                nodes.get_mut(start).unwrap().insert(current, path_len);
                continue;
            }

            if area.contains(&current) {
                for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    if !visited.contains(&(current.0 + dx, current.1 + dy))
                        && slopes.contains_key(&(current.0 + dx, current.1 + dy))
                        || area.contains(&(current.0 + dx, current.1 + dy))
                    {
                        queue.push_back(((current.0 + dx, current.1 + dy), path_len + 1));
                    }
                }
            } else if slopes.contains_key(&current) {
                let (dx, dy) = slopes.get(&current).unwrap();
                if !visited.contains(&(current.0 + dx, current.1 + dy)) {
                    queue.push_back(((current.0 + dx, current.1 + dy), path_len + 1));
                }
            }
        }
    }

    let start = nodes.keys().filter(|(_, y)| *y == 0).next().unwrap();
    let end = nodes
        .keys()
        .filter(|(_, y)| *y == contents.lines().count() as i64 - 1)
        .next()
        .unwrap();

    return longest_path(*start, *end, nodes.clone());
}

fn part2(contents: String) -> i64 {
    let mut area: HashSet<(i64, i64)> = HashSet::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => (),
                _ => {
                    area.insert((x as i64, y as i64));
                }
            }
        }
    }

    let mut nodes: HashMap<(i64, i64), HashMap<(i64, i64), i64>> = HashMap::new();
    for (x, y) in area.iter() {
        if *y == 0 || *y == contents.lines().count() as i64 - 1 {
            nodes.insert((*x, *y), HashMap::new());
            continue;
        }

        let mut neighbors: Vec<(i64, i64)> = Vec::new();
        for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if area.contains(&(x + dx, y + dy)) {
                neighbors.push((x + dx, y + dy));
            }
        }

        if neighbors.len() > 2 {
            nodes.insert((*x, *y), HashMap::new());
            continue;
        }
    }

    for start in nodes.clone().keys() {
        let mut queue: VecDeque<((i64, i64), i64)> = VecDeque::new();
        let mut visited: HashSet<(i64, i64)> = HashSet::new();
        queue.push_back((*start, 0));

        while let Some((current, path_len)) = queue.pop_front() {
            if visited.contains(&current) {
                continue;
            }

            visited.insert(current);

            if current != *start && nodes.contains_key(&current) {
                nodes.get_mut(start).unwrap().insert(current, path_len);
                continue;
            }

            for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                if !visited.contains(&(current.0 + dx, current.1 + dy))
                    && area.contains(&(current.0 + dx, current.1 + dy))
                {
                    queue.push_back(((current.0 + dx, current.1 + dy), path_len + 1));
                }
            }
        }
    }

    let binding = nodes.clone();
    let start = binding.keys().filter(|(_, y)| *y == 0).next().unwrap();
    let end = binding
        .keys()
        .filter(|(_, y)| *y == contents.lines().count() as i64 - 1)
        .next()
        .unwrap();

    for (node, connections) in nodes.clone() {
        if connections.len() > 3 {
            continue;
        }

        let mut min_dist = i64::MAX;
        let mut new_node: HashMap<(i64, i64), i64> = HashMap::new();
        let mut min_perimeter: Option<(i64, i64)> = None;

        for (n, q) in connections {
            if nodes.get(&n).unwrap().len() > 3 {
                new_node.insert(n, q);
                continue;
            }

            let dist = (n.0 - end.0).abs() + (n.1 - end.1).abs();
            if dist < min_dist {
                if min_perimeter.is_some() {
                    new_node.remove(&min_perimeter.unwrap());
                }

                min_perimeter = Some(n);
                min_dist = dist;
                new_node.insert(n, q);
            }
        }

        nodes.insert(node, new_node);
    }

    return longest_path(*start, *end, nodes.clone());
}

fn longest_path(
    start: (i64, i64),
    end: (i64, i64),
    neighbors: HashMap<(i64, i64), HashMap<(i64, i64), i64>>,
) -> i64 {
    let mut queue: VecDeque<((i64, i64), HashSet<(i64, i64)>, i64)> = VecDeque::new();
    queue.push_back((start, HashSet::from([start]), 0));

    let mut max_len: i64 = 0;
    while let Some((current, visited, path_len)) = queue.pop_front() {
        if current == end {
            max_len = max_len.max(path_len);
            continue;
        }

        for neighbor in neighbors.get(&current).unwrap() {
            if !visited.contains(neighbor.0) {
                let mut new_visited = visited.clone();
                new_visited.insert(*neighbor.0);
                queue.push_back((*neighbor.0, new_visited, path_len + neighbor.1));
            }
        }
    }

    return max_len;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 94);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 154);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "23".to_string();
	
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
        "\nPart 1:\nLongest Path: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nLongest Path: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}