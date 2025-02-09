use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut pos = (0, 0);
    let mut adjacencies: HashMap<(i64, i64), HashSet<(i64, i64)>> = HashMap::new();
    let mut x_set: HashSet<i64> = HashSet::new();
    let mut y_set: HashSet<i64> = HashSet::new();

    for line in contents.lines() {
        let past_pos = pos.clone();

        let d = line.chars().next().unwrap();
        let n = line[2..].split(" ").next().unwrap().parse::<i64>().unwrap();

        match d {
            'R' => pos.0 += n,
            'D' => pos.1 += n,
            'L' => pos.0 -= n,
            'U' => pos.1 -= n,
            _ => panic!("Invalid direction"),
        }

        adjacencies
            .entry(past_pos)
            .or_insert(HashSet::new())
            .insert(pos);
        adjacencies
            .entry(pos)
            .or_insert(HashSet::new())
            .insert(past_pos);


        x_set.insert(past_pos.0);
        y_set.insert(past_pos.1);
    }

    if pos != (0, 0) {
        panic!("Didn't end at 0, 0");
    }

    return enclosed_area(adjacencies, x_set, y_set);
}

fn part2(contents: String) -> i64 {
    let mut pos = (0, 0);
    let mut adjacencies: HashMap<(i64, i64), HashSet<(i64, i64)>> = HashMap::new();
    let mut x_set: HashSet<i64> = HashSet::new();
    let mut y_set: HashSet<i64> = HashSet::new();

    for line in contents.lines() {
        let past_pos = pos.clone();

        let color: String = line[2..]
            .split(" ")
            .skip(1)
            .next()
            .unwrap()
            .trim_start_matches("(#")
            .trim_end_matches(")")
            .to_string();
        let n: i64 = i64::from_str_radix(&color[..5].to_string(), 16).expect("Failed to parse hex");

        match color.chars().nth(5).unwrap() {
            '0' => pos.0 += n,
            '1' => pos.1 += n,
            '2' => pos.0 -= n,
            '3' => pos.1 -= n,
            _ => panic!("Invalid direction"),
        }

        adjacencies
            .entry(past_pos)
            .or_insert(HashSet::new())
            .insert(pos);
        adjacencies
            .entry(pos)
            .or_insert(HashSet::new())
            .insert(past_pos);

        x_set.insert(past_pos.0);
        y_set.insert(past_pos.1);
    }

    if pos != (0, 0) {
        panic!("Didn't end at 0, 0");
    }

    return enclosed_area(adjacencies, x_set, y_set);
}

fn enclosed_area(
    adjacencies: HashMap<(i64, i64), HashSet<(i64, i64)>>,
    x_set: HashSet<i64>,
    y_set: HashSet<i64>,
) -> i64 {
    let mut xs: Vec<i64> = x_set.iter().map(|x| *x).collect();
    let mut ys: Vec<i64> = y_set.iter().map(|y| *y).collect();

    xs.sort();
    ys.sort();

    let mut new_adjacencies: HashMap<(i64, i64), HashSet<(i64, i64)>> = HashMap::new();

    for (k, v) in adjacencies.iter() {
        let x_ix = xs.binary_search(&k.0).unwrap();
        let y_ix = ys.binary_search(&k.1).unwrap();

        for adj in v.iter() {
            let mut pos = k.clone();
            let end_ix;
            let mut ix;

            if k.0 == adj.0 {
                ix = y_ix;
                end_ix = ys.binary_search(&adj.1).unwrap();
            } else {
                ix = x_ix;
                end_ix = xs.binary_search(&adj.0).unwrap();
            }

            loop {
                let p_pos = pos.clone();
                ix = if ix < end_ix { ix + 1 } else { ix - 1 };

                if k.0 == adj.0 {
                    pos.1 = ys[ix];
                } else {
                    pos.0 = xs[ix];
                }

                new_adjacencies
                    .entry(p_pos)
                    .or_insert(HashSet::new())
                    .insert(pos.clone());
                new_adjacencies
                    .entry(pos)
                    .or_insert(HashSet::new())
                    .insert(p_pos.clone());

                if ix == end_ix {
                    break;
                }
            }
        }
    }

    let adjacencies = new_adjacencies;

    let mut count: i64 = 0;
    for i in 0..ys.len() {
        let mut inside_area: bool = false;
        let mut inside_start: i64 = 0;
        let y = ys[i];
        for j in 0..xs.len() {
            let mut new_inside_area: bool = inside_area;
            let x = xs[j];

            if i != 0 {
                // Handle rectangles above this line
                // Determine if inside the area
                if adjacencies.contains_key(&(x, y))
                    && adjacencies.get(&(x, y)).unwrap().contains(&(x, ys[i - 1]))
                {
                    if inside_area {
                        count += (x - inside_start + 1) * (y - ys[i - 1] - 1);
                    } else {
                        inside_start = x;
                    }

                    new_inside_area = !inside_area;
                }
            }

            if j != 0 {
                // Handle points on this line
                if inside_area
                    || (adjacencies.contains_key(&(x, y))
                        && adjacencies.get(&(x, y)).unwrap().contains(&(xs[j - 1], y)))
                {
                    count += x - xs[j - 1] - 1;
                }
            }

            count += (inside_area || adjacencies.contains_key(&(x, y))) as i64;
            inside_area = new_inside_area;
        }
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

        assert_eq!(part1(contents), 62);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 952408144115)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "18".to_string();
	
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
        "\nPart 1:\nEnclosed Area: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nEnclosed Area: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}