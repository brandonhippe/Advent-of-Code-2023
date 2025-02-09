use num::Integer;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let instructions: Vec<i32> = contents
        .lines()
        .nth(0)
        .expect("No lines in contents")
        .chars()
        .map(|c| {
            let mut d: i32;
            match c {
                'L' => d = 0,
                'R' => d = 1,
                _ => panic!("Invalid character"),
            }
            d
        })
        .collect();

    let mut connections: HashMap<String, Vec<String>> = HashMap::new();

    for line in contents.lines().skip(2) {
        let node: String = line
            .split(" = ")
            .nth(0)
            .expect("No lines in contents")
            .to_string();
        let node_connections: Vec<String> = line
            .split(" = ")
            .nth(1)

            .expect("No lines in contents")
            .split(", ")
            .map(|s| s.to_string().replace("(", "").replace(")", ""))
            .collect();

        connections.insert(node, node_connections);
    }

    let mut steps: i32 = 0;
    let mut node: String = "AAA".to_string();
    while node != "ZZZ" {
        node = connections.get(&node).expect("No lines in contents")
            [instructions[steps as usize % instructions.len()] as usize]
            .to_string();
        steps += 1;
    }

    return steps;
}

fn part2(contents: String) -> i64 {
    let instructions: Vec<i32> = contents
        .lines()
        .nth(0)
        .expect("No lines in contents")
        .chars()
        .map(|c| {
            let mut d: i32;
            match c {
                'L' => d = 0,
                'R' => d = 1,
                _ => panic!("Invalid character"),
            }
            d
        })
        .collect();

    let mut connections: HashMap<String, Vec<String>> = HashMap::new();

    for line in contents.lines().skip(2) {
        let node: String = line.split(" = ").nth(0).expect("No left node").to_string();
        let node_connections: Vec<String> = line
            .split(" = ")
            .nth(1)
            .expect("No connections")
            .split(", ")
            .map(|s| s.to_string().replace("(", "").replace(")", ""))
            .collect();

        connections.insert(node, node_connections);
    }

    let a_nodes: Vec<String> = connections
        .keys()
        .filter(|s| s.chars().nth(2).expect("Node not of length 3") == 'A')
        .map(|s| s.to_string())
        .collect();

    let mut steps: i64 = 1;

    for start_node in a_nodes {
        let mut node_steps: i64 = 0;
        let mut node: String = start_node.to_string();
        while node.chars().nth(2).expect("Node not of length 3") != 'Z' {
            node = connections.get(&node).expect("No connections")
                [instructions[node_steps as usize % instructions.len()] as usize]
                .to_string();
            node_steps += 1;
        }

        steps = steps.lcm(&node_steps);
    }

    return steps;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("p1_example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 6);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("p2_example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 6);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "8".to_string();
	
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
        "\nPart 1:\nSteps to reach ZZZ: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSteps to be only on nodes ending in Z: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}