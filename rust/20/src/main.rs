use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let mut modules: HashMap<String, Module> = HashMap::new();
    for line in contents.lines() {
        let to_modules: Vec<String> = line
            .split(" -> ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|x| x.to_string())
            .collect();
        match line.chars().nth(0).unwrap() {
            '%' => modules.insert(
                line.split(" -> ").nth(0).unwrap().chars().skip(1).collect(),
                Module {
                    module_type: '%',
                    to_modules: to_modules,
                    from_modules: HashMap::new(),
                    from_vals: 0,
                },
            ),
            '&' => modules.insert(
                line.split(" -> ").nth(0).unwrap().chars().skip(1).collect(),
                Module {
                    module_type: '&',
                    to_modules: to_modules,
                    from_modules: HashMap::new(),
                    from_vals: 0,

                },
            ),
            _ => modules.insert(
                line.split(" -> ").nth(0).unwrap().to_string(),
                Module {
                    module_type: ' ',
                    to_modules: to_modules,
                    from_modules: HashMap::new(),
                    from_vals: 0,
                },
            ),
        };
    }

    for (name, module) in modules.clone().iter() {
        for to_module in module.to_modules.iter() {
            if !modules.contains_key(to_module) {
                continue;
            }

            let to_from: &mut Module = modules.get_mut(to_module).unwrap();
            if to_from.module_type == '&' {
                to_from.from_vals += 1 << (to_from.from_modules.len() as i32);
            }

            to_from
                .from_modules
                .insert(name.to_string(), 1 << (to_from.from_modules.len() as i32));
        }
    }

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _pulses in 0..1000 {
        let mut queue: VecDeque<(String, String, bool)> = VecDeque::from(vec![(
            "broadcaster".to_string(),
            "button".to_string(),
            false,
        )]);
        while let Some((to_name, from_name, pulse)) = queue.pop_front() {
            if pulse {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }

            if !modules.contains_key(&to_name) {
                continue;
            }

            let to_module = modules.get_mut(&to_name).unwrap();

            match to_module.module_type {
                '%' => {
                    if !pulse {
                        if to_module.from_vals == 0 {
                            to_module.from_vals = 1;
                            for t_name in to_module.to_modules.iter() {
                                queue.push_back((t_name.to_string(), to_name.to_string(), true));
                            }
                        } else {
                            to_module.from_vals = 0;
                            for t_name in to_module.to_modules.iter() {
                                queue.push_back((t_name.to_string(), to_name.to_string(), false));
                            }
                        }
                    }
                }
                '&' => {
                    if pulse {
                        if to_module.from_vals & to_module.from_modules.get(&from_name).unwrap()
                            != 0
                        {
                            to_module.from_vals -= to_module.from_modules.get(&from_name).unwrap();
                        }
                    } else {
                        if to_module.from_vals & to_module.from_modules.get(&from_name).unwrap()
                            == 0
                        {
                            to_module.from_vals += to_module.from_modules.get(&from_name).unwrap();
                        }
                    }

                    for t_name in to_module.to_modules.iter() {
                        queue.push_back((
                            t_name.to_string(),
                            to_name.to_string(),
                            to_module.from_vals != 0,
                        ));
                    }
                }
                _ => {
                    for t_name in to_module.to_modules.iter() {
                        queue.push_back((t_name.to_string(), to_name.to_string(), pulse));
                    }
                }
            }
        }
    }

    return low_pulses * high_pulses;
}

fn part2(contents: String) -> i64 {
    let mut modules: HashMap<String, Module> = HashMap::new();
    for line in contents.lines() {
        let to_modules: Vec<String> = line
            .split(" -> ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|x| x.to_string())
            .collect();
        match line.chars().nth(0).unwrap() {
            '%' => modules.insert(
                line.split(" -> ").nth(0).unwrap().chars().skip(1).collect(),
                Module {
                    module_type: '%',
                    to_modules: to_modules,
                    from_modules: HashMap::new(),
                    from_vals: 0,
                },
            ),
            '&' => modules.insert(
                line.split(" -> ").nth(0).unwrap().chars().skip(1).collect(),
                Module {
                    module_type: '&',
                    to_modules: to_modules,
                    from_modules: HashMap::new(),
                    from_vals: 0,
                },
            ),
            _ => modules.insert(
                line.split(" -> ").nth(0).unwrap().to_string(),
                Module {
                    module_type: ' ',
                    to_modules: to_modules,
                    from_modules: HashMap::new(),
                    from_vals: 0,
                },
            ),
        };
    }

    for (name, module) in modules.clone().iter() {
        for to_module in module.to_modules.iter() {
            if !modules.contains_key(to_module) {
                continue;
            }

            let to_from: &mut Module = modules.get_mut(to_module).unwrap();
            if to_from.module_type == '&' {
                to_from.from_vals += 1 << (to_from.from_modules.len() as i32);
            }

            to_from
                .from_modules
                .insert(name.to_string(), 1 << (to_from.from_modules.len() as i32));
        }
    }

    let mut common: HashSet<String> = HashSet::from_iter(modules.keys().cloned());
    for name in modules.get("broadcaster").unwrap().to_modules.iter() {
        let mut part_modules: HashSet<String> = HashSet::new();

        let mut open_modules: VecDeque<String> = VecDeque::from(vec![name.to_string()]);
        while let Some(m_name) = open_modules.pop_front() {
            part_modules.insert(m_name.to_string());
            if !modules.contains_key(&m_name) {
                continue;
            }

            for t_name in modules.get(&m_name).unwrap().to_modules.iter() {
                if !part_modules.contains(t_name) {
                    open_modules.push_back(t_name.to_string());
                }
            }
        }

        common = common.intersection(&part_modules).cloned().collect();
    }

    let mut track_outputs: HashMap<String, Vec<i32>> = HashMap::from_iter(
        modules
            .get(common.iter().next().unwrap())
            .unwrap()
            .from_modules
            .keys()
            .map(|name| (name.to_string(), vec![])),
    );
    let mut presses = 0;

    while track_outputs.values().any(|x| x.len() < 2) {
        presses += 1;
        let mut queue: VecDeque<(String, String, bool)> = VecDeque::from(vec![(
            "broadcaster".to_string(),
            "button".to_string(),
            false,
        )]);
        while let Some((to_name, from_name, pulse)) = queue.pop_front() {
            if pulse && track_outputs.contains_key(&from_name) {
                track_outputs.get_mut(&from_name).unwrap().push(presses);
            }

            if !modules.contains_key(&to_name) {
                continue;
            }

            let to_module = modules.get_mut(&to_name).unwrap();

            match to_module.module_type {
                '%' => {
                    if !pulse {
                        if to_module.from_vals == 0 {
                            to_module.from_vals = 1;
                            for t_name in to_module.to_modules.iter() {
                                queue.push_back((t_name.to_string(), to_name.to_string(), true));
                            }
                        } else {
                            to_module.from_vals = 0;
                            for t_name in to_module.to_modules.iter() {
                                queue.push_back((t_name.to_string(), to_name.to_string(), false));
                            }
                        }
                    }
                }
                '&' => {
                    if pulse {
                        if to_module.from_vals & to_module.from_modules.get(&from_name).unwrap()
                            != 0
                        {
                            to_module.from_vals -= to_module.from_modules.get(&from_name).unwrap();
                        }
                    } else {
                        if to_module.from_vals & to_module.from_modules.get(&from_name).unwrap()
                            == 0
                        {
                            to_module.from_vals += to_module.from_modules.get(&from_name).unwrap();
                        }
                    }

                    for t_name in to_module.to_modules.iter() {
                        queue.push_back((
                            t_name.to_string(),
                            to_name.to_string(),
                            to_module.from_vals != 0,
                        ));
                    }
                }
                _ => {
                    for t_name in to_module.to_modules.iter() {
                        queue.push_back((t_name.to_string(), to_name.to_string(), pulse));
                    }
                }
            }
        }
    }

    return track_outputs
        .values()
        .map(|x| (x[1] - x[0]) as i64)
        .product::<i64>();
}

#[derive(Clone, Debug)]
struct Module {
    module_type: char,
    to_modules: Vec<String>,
    from_modules: HashMap<String, i32>,
    from_vals: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example_1.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 32000000);

        let contents =
            fs::read_to_string("example_2.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 11687500);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "20".to_string();
	
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
        "\nPart 1:\nTotal low pulses * total high pulses: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nButton presses to send single rx to output: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}