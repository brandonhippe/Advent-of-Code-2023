use relative_path::RelativePath;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let mut rules: HashMap<String, Vec<Condition>> = HashMap::new();
    for line in contents.lines() {
        if line.len() == 0 {
            break;
        }

        let name: String = line.split("{").nth(0).unwrap().to_string();
        let rs: Vec<Condition> = line
            .split("{")
            .nth(1)
            .unwrap()
            .replace("}", "")
            .split(",")
            .map(|part| {
                let cond = part.to_string();
                let this_condition: Condition;
                if cond.contains(":") {
                    let condition: String = cond.split(":").nth(0).unwrap().to_string();
                    let true_dest: String = cond.split(":").nth(1).unwrap().to_string();

                    let spec = condition.chars().nth(0).unwrap();
                    let test = condition.chars().nth(1).unwrap();
                    let val = condition
                        .chars()
                        .skip(2)
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap();


                    this_condition = Condition { 
                        spec: Some(spec),
                        test: Some(test),
                        val: Some(val),
                        true_dest: true_dest
                    }
                } else {
                    this_condition = Condition {
                        spec: None,
                        test: None,
                        val: None,
                        true_dest: cond
                    }
                }

                this_condition
            }).collect();

        rules.insert(name, rs);
    }

    let mut acccepted_sum: i32 = 0;
    for line in contents.lines().skip(rules.len() + 1) {
        let specs: HashMap<char, i32> =
            HashMap::from_iter(line[1..line.len() - 1].split(",").map(|part| {
                (
                    part.split('=').nth(0).unwrap().chars().nth(0).unwrap(),
                    part.split('=').nth(1).unwrap().parse::<i32>().unwrap(),
                )
            }));

        let mut curr_rule: String = "in".to_string();
        let accepted: bool;
        loop {
            if curr_rule == "A" {
                accepted = true;
                break;
            }

            if curr_rule == "R" {
                accepted = false;
                break;
            }

            for rule in rules.get(&curr_rule).unwrap() {
                if rule.spec.is_some() {
                    if rule.test.unwrap() == '<' {
                        if specs.get(&rule.spec.unwrap()).unwrap() < &rule.val.unwrap() {
                            curr_rule = rule.true_dest.clone();
                            break;
                        }
                    } else {
                        if specs.get(&rule.spec.unwrap()).unwrap() > &rule.val.unwrap() {
                            curr_rule = rule.true_dest.clone();
                            break;
                        }
                    }
                } else {
                    curr_rule = rule.true_dest.clone();
                }
            }
        }

        if accepted {
            acccepted_sum += specs.values().sum::<i32>();
        }
    }

    return acccepted_sum;
}

fn part2(contents: String) -> i64 {
    let mut rules: HashMap<String, Vec<Condition>> = HashMap::new();
    for line in contents.lines() {
        if line.len() == 0 {
            break;
        }

        let name: String = line.split("{").nth(0).unwrap().to_string();
        let rs: Vec<Condition> = line
            .split("{")
            .nth(1)
            .unwrap()
            .replace("}", "")
            .split(",")
            .map(|part| {
                let cond = part.to_string();
                let this_condition: Condition;
                if cond.contains(":") {
                    let condition: String = cond.split(":").nth(0).unwrap().to_string();
                    let true_dest: String = cond.split(":").nth(1).unwrap().to_string();

                    let spec = condition.chars().nth(0).unwrap();
                    let test = condition.chars().nth(1).unwrap();
                    let val = condition
                        .chars()
                        .skip(2)
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap();

                    this_condition = Condition { 
                        spec: Some(spec),
                        test: Some(test),
                        val: Some(val),
                        true_dest: true_dest
                    }
                } else {
                    this_condition = Condition {
                        spec: None,
                        test: None,
                        val: None,
                        true_dest: cond
                    }
                }

                this_condition
            }).collect();

        rules.insert(name, rs);
    }

    let accepted = rule_combs(rules, "in".to_string());

    let mut total: i64 = 0;
    for i in 0..accepted.get(&'x').unwrap().len() {
        total += accepted
            .values()
            .map(|conds| conds[i].1 as i64 - conds[i].0 as i64 + 1)
            .product::<i64>();
    }

    return total;
}

#[derive(Clone)]
struct Condition {
    spec: Option<char>,
    test: Option<char>,
    val: Option<i32>,
    true_dest: String,
}

fn rule_combs(rules: HashMap<String, Vec<Condition>>, rule: String) -> HashMap<char, Vec<(i32, i32)>> {
    if rule == "A".to_string() {
        return HashMap::from_iter(vec![
            ('x', vec![(1, 4000)]),
            ('m', vec![(1, 4000)]),
            ('a', vec![(1, 4000)]),
            ('s', vec![(1, 4000)]),
        ]);
    }

    if rule == "R".to_string() {
        return HashMap::from_iter(vec![
            ('x', vec![]),
            ('m', vec![]),
            ('a', vec![]),
            ('s', vec![]),
        ]);
    }

    let mut rule_conds: HashMap<char, Vec<(i32, i32)>> = HashMap::from_iter(vec![
        ('x', vec![]),
        ('m', vec![]),
        ('a', vec![]),
        ('s', vec![]),
    ]);

    for r in rules.get(&rule).unwrap().iter().rev() {
        let mut dest_conds: HashMap<char, Vec<(i32, i32)>>;

        if r.spec.is_some() {
            let spec = r.spec.unwrap();
            let test = r.test.unwrap();
            let val = r.val.unwrap();
            let true_dest = r.true_dest.clone();

            let cond: HashMap<char, Vec<(i32, i32)>>;
            let not_cond: HashMap<char, Vec<(i32, i32)>>;
            if test == '<' {
                cond = HashMap::from_iter(vec![(spec, vec![(1, val - 1)])]);
                not_cond = HashMap::from_iter(vec![(spec, vec![(val, 4000)])]);
            } else {
                cond = HashMap::from_iter(vec![(spec, vec![(val + 1, 4000)])]);
                not_cond = HashMap::from_iter(vec![(spec, vec![(1, val)])]);
            }

            rule_conds = and_rule_conds(rule_conds.clone(), not_cond);
            dest_conds = rule_combs(rules.clone(), true_dest);
            dest_conds = and_rule_conds(dest_conds.clone(), cond);
        } else {
            dest_conds = rule_combs(rules.clone(), r.true_dest.clone());
        }

        rule_conds = or_rule_conds(rule_conds.clone(), dest_conds);
    }

    return rule_conds;
}

fn or_rule_conds(
    conds1: HashMap<char, Vec<(i32, i32)>>,
    conds2: HashMap<char, Vec<(i32, i32)>>,
) -> HashMap<char, Vec<(i32, i32)>> {
    let mut new_conds: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (spec, conds) in conds1.iter() {
        new_conds
            .entry(*spec)
            .or_insert(vec![])
            .extend(conds.clone());
    }

    for (spec, conds) in conds2.iter() {
        new_conds
            .entry(*spec)
            .or_insert(vec![])
            .extend(conds.clone());
    }

    return new_conds;
}

fn and_rule_conds(
    conds1: HashMap<char, Vec<(i32, i32)>>,
    conds2: HashMap<char, Vec<(i32, i32)>>,
) -> HashMap<char, Vec<(i32, i32)>> {
    let mut new_conds: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (spec, conds) in conds1.iter() {
        let new_cond = new_conds.entry(*spec).or_insert(vec![]);

        if !conds2.contains_key(&spec) || conds2.get(&spec).unwrap().len() == 0 {
            new_cond.extend(conds.clone());
            continue;
        }

        for (min1, max1) in conds.iter() {
            for (min2, max2) in conds2.get(&spec).unwrap() {
                new_cond.push((std::cmp::max(*min1, *min2), std::cmp::min(*max1, *max2)));
            }
        }
    }

    return new_conds;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 19114);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 167409079868000)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "19".to_string();
	
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
        "\nPart 1:\nSum of rating numbers of accepted parts: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nTotal number of accepted combinations: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}