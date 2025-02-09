use itertools::Itertools;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut sands: Vec<((i64, i64, i64), (i64, i64, i64))> =
        Vec::from_iter(contents.lines().map(|line| {
            line.split("~")
                .map(|pos| {
                    pos.split(",")
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        }));

    sands.sort_by(|a, b| a.0 .2.cmp(&b.0 .2));

    let mut max_heights: HashMap<(i64, i64), (i64, usize)> = HashMap::new();
    let mut on_top_of: HashMap<usize, HashSet<usize>> = HashMap::new();

    for ix in 0..sands.len() {
        let x_min = sands[ix].0 .0;
        let x_max = sands[ix].1 .0;
        let y_min = sands[ix].0 .1;
        let y_max = sands[ix].1 .1;
        let z_min = sands[ix].0 .2;
        let z_max = sands[ix].1 .2;


        let mut new_height = 1;

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let (h, h_ix) = max_heights.get(&(x, y)).unwrap_or(&(-1, usize::MAX));
                if *h + 1 > new_height {
                    new_height = *h + 1;
                    on_top_of.remove(&ix);
                }

                if *h + 1 == new_height && new_height > 1 {
                    on_top_of.entry(ix).or_insert(HashSet::new()).insert(*h_ix);
                }
            }
        }

        sands[ix].0 = (x_min, y_min, new_height);
        sands[ix].1 = (x_max, y_max, new_height + z_max - z_min);

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                max_heights.insert((x, y), (new_height + z_max - z_min, ix));
            }
        }
    }

    let mut safe: HashSet<usize> = HashSet::from_iter(0..sands.len());
    for on_top in on_top_of.values() {
        if on_top.len() == 1 {
            safe = safe.difference(&on_top).map(|x| *x).collect();
        }
    }

    return safe.len() as i64;
}

fn part2(contents: String) -> i64 {
    let mut sands: Vec<((i64, i64, i64), (i64, i64, i64))> =
        Vec::from_iter(contents.lines().map(|line| {
            line.split("~")
                .map(|pos| {
                    pos.split(",")
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        }));

    sands.sort_by(|a, b| a.0 .2.cmp(&b.0 .2));

    let mut max_heights: HashMap<(i64, i64), (i64, usize)> = HashMap::new();
    let mut on_top_of: HashMap<usize, HashSet<usize>> = HashMap::new();

    for ix in 0..sands.len() {
        let x_min = sands[ix].0 .0;
        let x_max = sands[ix].1 .0;
        let y_min = sands[ix].0 .1;
        let y_max = sands[ix].1 .1;
        let z_min = sands[ix].0 .2;
        let z_max = sands[ix].1 .2;

        let mut new_height = 1;

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let (h, h_ix) = max_heights.get(&(x, y)).unwrap_or(&(-1, usize::MAX));
                if *h + 1 > new_height {
                    new_height = *h + 1;
                    on_top_of.remove(&ix);
                }

                if *h + 1 == new_height && new_height > 1 {
                    on_top_of.entry(ix).or_insert(HashSet::new()).insert(*h_ix);
                }
            }
        }

        sands[ix].0 = (x_min, y_min, new_height);
        sands[ix].1 = (x_max, y_max, new_height + z_max - z_min);

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                max_heights.insert((x, y), (new_height + z_max - z_min, ix));
            }
        }
    }

    let mut safe: HashSet<usize> = HashSet::from_iter(0..sands.len());
    for on_top in on_top_of.values() {
        if on_top.len() == 1 {
            safe = safe.difference(&on_top).map(|x| *x).collect();
        }
    }

    let mut underneath: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (ix, on_top) in on_top_of.iter() {
        for on in on_top.iter() {
            underneath.entry(*on).or_insert(HashSet::new()).insert(*ix);
        }
    }

    let to_disintegrate = HashSet::from_iter(0..sands.len())
        .difference(&safe)
        .map(|x| *x)
        .collect::<Vec<usize>>();
    let mut count: i64 = 0;

    for dis_ix in to_disintegrate {
        let mut disintegrated: HashSet<usize> = HashSet::new();
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(dis_ix);

        while let Some(ix) = queue.pop_front() {
            disintegrated.insert(ix);

            for under in underneath.get(&ix).unwrap_or(&HashSet::new()).iter() {
                if on_top_of
                    .get(under)
                    .unwrap()
                    .iter()
                    .all(|d| disintegrated.contains(d))
                {
                    queue.push_back(*under);
                }
            }
        }

        count += disintegrated.len() as i64 - 1;
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 5);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 7);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "22".to_string();
	
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
        "\nPart 1:\nNumber of safe blocks: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nTotal consequential disintegrations: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}