use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn part1(contents: String) -> i32 {
    let mut connections: HashMap<(i32, i32), HashMap<(i32, i32), Vec<(i32, i32)>>> = HashMap::new();

    for (i, line) in contents.lines().enumerate() {
        let y: i32 = i as i32;
        for (j, c) in line.chars().enumerate() {
            let x: i32 = j as i32;
            let mut this_connections: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
            match c {
                '.' => {
                    this_connections.insert((x - 1, y), vec![(x + 1, y)]);
                    this_connections.insert((x + 1, y), vec![(x - 1, y)]);
                    this_connections.insert((x, y - 1), vec![(x, y + 1)]);
                    this_connections.insert((x, y + 1), vec![(x, y - 1)]);
                },
                '/' => {
                    this_connections.insert((x - 1, y), vec![(x, y - 1)]);
                    this_connections.insert((x, y - 1), vec![(x - 1, y)]);
                    this_connections.insert((x + 1, y), vec![(x, y + 1)]);
                    this_connections.insert((x, y + 1), vec![(x + 1, y)]);
                }
                '\\' => {
                    this_connections.insert((x - 1, y), vec![(x, y + 1)]);
                    this_connections.insert((x, y + 1), vec![(x - 1, y)]);
                    this_connections.insert((x + 1, y), vec![(x, y - 1)]);
                    this_connections.insert((x, y - 1), vec![(x + 1, y)]);
                },

                '|' => {
                    this_connections.insert((x, y - 1), vec![(x, y + 1)]);
                    this_connections.insert((x, y + 1), vec![(x, y - 1)]);
                    this_connections.insert((x - 1, y), vec![(x, y + 1), (x, y - 1)]);
                    this_connections.insert((x + 1, y), vec![(x, y + 1), (x, y - 1)]);
                },
                '-' => {
                    this_connections.insert((x - 1, y), vec![(x + 1, y)]);
                    this_connections.insert((x + 1, y), vec![(x - 1, y)]);
                    this_connections.insert((x, y - 1), vec![(x + 1, y), (x - 1, y)]);
                    this_connections.insert((x, y + 1), vec![(x + 1, y), (x - 1, y)]);
                },
                _ => {}
            }

            connections.insert((x, y), this_connections);
        }
    }

    let min_coord = 0;
    let max_coord = contents.lines().count() as i32 - 1;

    let mut visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
    let mut queue: VecDeque<((i32, i32), (i32, i32))> = VecDeque::from(vec![((0, 0), (-1, 0))]);

    while let Some((pos, from)) = queue.pop_front() {
        if visited.contains(&(pos, from)) {
            continue;
        }

        if pos.0 < min_coord || pos.0 > max_coord || pos.1 < min_coord || pos.1 > max_coord {
            continue;
        }

        visited.insert((pos, from));

        let this_connections = connections.get(&pos).unwrap();
        for next in this_connections.get(&from).unwrap().clone().iter() {
            if visited.contains(&(next.clone(), pos)) {
                continue;
            }

            queue.push_back((next.clone(), pos.clone()));
        }
    }

    let visited_positions: HashSet<(i32, i32)> = HashSet::from_iter(visited.iter().map(|(pos, _)| pos.clone()));
    return visited_positions.len() as i32;
}

fn part2(contents: String) -> i32 {
    let mut connections: HashMap<(i32, i32), HashMap<(i32, i32), Vec<(i32, i32)>>> = HashMap::new();
    let mut splitters: HashSet<(i32, i32)> = HashSet::new();

    for (i, line) in contents.lines().enumerate() {
        let y: i32 = i as i32;
        for (j, c) in line.chars().enumerate() {
            let x: i32 = j as i32;
            let mut this_connections: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
            match c {
                '.' => {
                    this_connections.insert((x - 1, y), vec![(x + 1, y)]);
                    this_connections.insert((x + 1, y), vec![(x - 1, y)]);
                    this_connections.insert((x, y - 1), vec![(x, y + 1)]);
                    this_connections.insert((x, y + 1), vec![(x, y - 1)]);
                },
                '/' => {
                    this_connections.insert((x - 1, y), vec![(x, y - 1)]);
                    this_connections.insert((x, y - 1), vec![(x - 1, y)]);
                    this_connections.insert((x + 1, y), vec![(x, y + 1)]);
                    this_connections.insert((x, y + 1), vec![(x + 1, y)]);
                }
                '\\' => {
                    this_connections.insert((x - 1, y), vec![(x, y + 1)]);
                    this_connections.insert((x, y + 1), vec![(x - 1, y)]);
                    this_connections.insert((x + 1, y), vec![(x, y - 1)]);
                    this_connections.insert((x, y - 1), vec![(x + 1, y)]);
                },
                '|' => {
                    this_connections.insert((x, y - 1), vec![(x, y + 1)]);
                    this_connections.insert((x, y + 1), vec![(x, y - 1)]);
                    this_connections.insert((x - 1, y), vec![(x, y + 1), (x, y - 1)]);
                    this_connections.insert((x + 1, y), vec![(x, y + 1), (x, y - 1)]);
                    splitters.insert((x, y));
                },
                '-' => {
                    this_connections.insert((x - 1, y), vec![(x + 1, y)]);
                    this_connections.insert((x + 1, y), vec![(x - 1, y)]);
                    this_connections.insert((x, y - 1), vec![(x + 1, y), (x - 1, y)]);
                    this_connections.insert((x, y + 1), vec![(x + 1, y), (x - 1, y)]);
                    splitters.insert((x, y));
                },
                _ => {}
            }

            connections.insert((x, y), this_connections);
        }
    }

    let min_coord = 0;
    let max_coord = contents.lines().count() as i32 - 1;
    
    let mut edges: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

    for sp in splitters.iter() {
        for (from, to_vec) in connections.get(sp).unwrap() {
            if to_vec.len() == 1 {
                continue;
            }

            let mut pos: (i32, i32) = sp.clone();
            let mut next: (i32, i32) = from.clone();
            loop {
                if !connections.contains_key(&next) {
                    edges.insert((pos, next));
                    break;
                }
                
                let mut new_pos: Option<(i32, i32)> = None;
                let mut new_next: Option<(i32, i32)> = None;
                
                let this_connections = connections.get(&next).unwrap();
                
                for (pos_next, from_vec) in this_connections.iter() {
                    if !from_vec.contains(&pos) {
                        continue;
                    }
                    
                    if from_vec.len() > 1 {
                        new_pos = None;
                        new_next = None;
                        break;
                    }
                    
                    new_pos = Some(next.clone());
                    new_next = Some(pos_next.clone());
                }

                if !(new_pos.and(new_next).is_some()) {
                    break;
                }

                pos = new_pos.unwrap();
                next = new_next.unwrap();
            }
        }  
    }

    let mut max_energized: i32 = 0;

    for (edge, from) in edges.iter() {
        let mut visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
        let mut queue: VecDeque<((i32, i32), (i32, i32))> = VecDeque::from(vec![(*edge, *from)]);

        while let Some((pos, from)) = queue.pop_front() {
            if visited.contains(&(pos, from)) {
                continue;
            }

            if pos.0 < min_coord || pos.0 > max_coord || pos.1 < min_coord || pos.1 > max_coord {
                continue;
            }

            visited.insert((pos, from));

            let this_connections = connections.get(&pos).unwrap();
            for next in this_connections.get(&from).unwrap().clone().iter() {
                if visited.contains(&(next.clone(), pos)) {
                    continue;
                }

                queue.push_back((next.clone(), pos.clone()));
            }
        }

        let visited_positions: HashSet<(i32, i32)> = HashSet::from_iter(visited.iter().map(|(pos, _)| pos.clone()));
        max_energized = std::cmp::max(max_energized, visited_positions.len() as i32);
    }

    return max_energized;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 46);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 51)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "16".to_string();
	
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
        "\nPart 1:\nTiles energized: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nMaximum tiles energized: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}