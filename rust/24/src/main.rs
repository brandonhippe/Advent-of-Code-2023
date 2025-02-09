use itertools::Itertools;
use regex::Regex;
use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String, min_c: f64, max_c: f64) -> i64 {
    let lines: Vec<(f64, f64, f64, f64)> = contents
        .lines()
        .map(|line| {
            let ps_vs: Vec<&str> = line.split(" @ ").collect::<Vec<&str>>();
            let (px, py): (f64, f64) = (
                ps_vs[0].split(", ").collect::<Vec<&str>>()[0]
                    .trim()
                    .parse()
                    .unwrap(),
                ps_vs[0].split(", ").collect::<Vec<&str>>()[1]
                    .trim()
                    .parse()
                    .unwrap(),
            );
            let (vx, vy): (f64, f64) = (
                ps_vs[1].split(", ").collect::<Vec<&str>>()[0]
                    .trim()
                    .parse()
                    .unwrap(),
                ps_vs[1].split(", ").collect::<Vec<&str>>()[1]
                    .trim()
                    .parse()
                    .unwrap(),
            );

            (-1.0 / vx, 1.0 / vy, py / vy, -px / vx)
        })

        .collect();

    let mut intersections: i64 = 0;
    for i in 0..lines.len() - 1 {
        for j in i + 1..lines.len() {
            let (x, y) = intersect_2d(
                (lines[i].0, lines[i].1, lines[i].2 + lines[i].3),
                (lines[j].0, lines[j].1, lines[j].2 + lines[j].3),
            );
            if x >= min_c && x <= max_c && y >= min_c && y <= max_c {
                let t1 = -x * lines[i].0 + lines[i].3;
                let t2 = -x * lines[j].0 + lines[j].3;

                intersections += (t1 >= 0.0 && t2 >= 0.0) as i64;
            }
        }
    }

    return intersections;
}

fn part2(contents: String) -> i64 {
    let re_nums = Regex::new(r"(-?\d+)").unwrap();
    let mut rock_sum: f64 = 0.0;

    for start_ix in 0..contents.lines().count() - 3 {
        let (px0, py0, pz0, vx0, vy0, vz0) = re_nums
            .captures_iter(contents.lines().nth(start_ix + 0).unwrap())
            .map(|cap| cap[1].parse::<f64>().unwrap())
            .collect_tuple()
            .unwrap();
        let (px1, py1, pz1, vx1, vy1, vz1) = re_nums
            .captures_iter(contents.lines().nth(start_ix + 1).unwrap())
            .map(|cap| cap[1].parse::<f64>().unwrap())
            .collect_tuple()
            .unwrap();
        let (px2, py2, pz2, vx2, vy2, vz2) = re_nums
            .captures_iter(contents.lines().nth(start_ix + 2).unwrap())
            .map(|cap| cap[1].parse::<f64>().unwrap())
            .collect_tuple()
            .unwrap();
        let (px3, py3, pz3, vx3, vy3, vz3) = re_nums
            .captures_iter(contents.lines().nth(start_ix + 3).unwrap())
            .map(|cap| cap[1].parse::<f64>().unwrap())
            .collect_tuple()
            .unwrap();

        let system = vec![
            vec![vy0 - vy1, vx1 - vx0, 0.0, py1 - py0, px0 - px1, 0.0],
            vec![vz0 - vz1, 0.0, vx1 - vx0, pz1 - pz0, 0.0, px0 - px1],
            vec![vy0 - vy2, vx2 - vx0, 0.0, py2 - py0, px0 - px2, 0.0],
            vec![vz0 - vz2, 0.0, vx2 - vx0, pz2 - pz0, 0.0, px0 - px2],
            vec![vy0 - vy3, vx3 - vx0, 0.0, py3 - py0, px0 - px3, 0.0],
            vec![vz0 - vz3, 0.0, vx3 - vx0, pz3 - pz0, 0.0, px0 - px3],
        ];

        let det = determinant(system.clone());
        if det == 0.0 || det.is_nan() {
            continue;
        }

        let sol_vec = vec![
            px0 * vy0 - py0 * vx0 - px1 * vy1 + py1 * vx1,
            px0 * vz0 - pz0 * vx0 - px1 * vz1 + pz1 * vx1,
            px0 * vy0 - py0 * vx0 - px2 * vy2 + py2 * vx2,
            px0 * vz0 - pz0 * vx0 - px2 * vz2 + pz2 * vx2,
            px0 * vy0 - py0 * vx0 - px3 * vy3 + py3 * vx3,
            px0 * vz0 - pz0 * vx0 - px3 * vz3 + pz3 * vx3,
        ];

        for i in 0..3 {
            let mut new_system = system.clone();
            for j in 0..6 {
                new_system[j][i] = sol_vec[j];
            }

            rock_sum += determinant(new_system) / det;
        }

        break;
    }

    return rock_sum as i64;
}

fn intersect_2d(l1: (f64, f64, f64), l2: (f64, f64, f64)) -> (f64, f64) {
    let (a1, b1, c1) = l1;
    let (a2, b2, c2) = l2;

    let d = a1 * b2 - a2 * b1;
    let dx = c1 * b2 - c2 * b1;
    let dy = a1 * c2 - a2 * c1;

    return (dx / d, dy / d);
}

fn determinant(a1: Vec<Vec<f64>>) -> f64 {
    let mut a = a1.clone();
    for i in 0..a.len() {
        let first_val = a[i][i];
        for j in 0..a.len() {
            if j == i {
                continue;
            }

            let mult_amt = a[j][i] / first_val;

            for k in 0..a.len() {
                a[j][k] -= a[i][k] * mult_amt;
            }
        }
    }

    return (0..a.len()).map(|i| a[i][i]).product();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents, 7.0, 27.0), 2);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 47);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "24".to_string();
	
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
        "\nPart 1:\nNumber of 2d intersections: {}\nRan in {:.5?}",
        part1(contents.clone(), 200000000000000.0, 400000000000000.0),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSum of Coordinates of Rock Throw: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}