use ndarray::Array2;
use numpy::IntoPyArray;
use pyo3::prelude::*;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    for line in contents.lines() {
        let s1: String = line.split(":").nth(0).unwrap().to_string();
        let others: Vec<String> = line
            .split(": ")
            .nth(1)
            .unwrap()
            .split(" ")
            .map(|s| s.to_string())
            .collect();

        for other in others {
            connections
                .entry(s1.clone())
                .or_insert(HashSet::new())
                .insert(other.clone());
            connections
                .entry(other.clone())
                .or_insert(HashSet::new())
                .insert(s1.clone());
        }
    }

    let arr_dim = connections.len();

    let mut degree: Array2<f64> = Array2::zeros((arr_dim, arr_dim));
    let mut adj: Array2<f64> = Array2::zeros((arr_dim, arr_dim));
    let mapping: Vec<String> = connections.keys().cloned().collect();

    for (i, k) in mapping.iter().enumerate() {
        degree[(i, i)] = connections.get(k).unwrap().len() as f64;

        for n in connections.get(k).unwrap() {
            let j = mapping.iter().position(|x| x == n).unwrap();
            adj[(j, i)] = 1.0;
        }
    }

    let laplacian = degree - adj;
    pyo3::prepare_freethreaded_python();

    let fiedler_vector: Vec<f64> = Python::with_gil(|py| {
        let np_lin_alg = PyModule::import(py, "numpy.linalg")?;
        let laplacian_py = laplacian.into_pyarray(py).to_object(py);
        let vh = np_lin_alg
            .getattr("svd")?
            .call1((laplacian_py,))?
            .getattr("Vh")?;
        let fiedler_vector = vh.get_item(-2)?.to_object(py);
        fiedler_vector.extract::<Vec<f64>>(py)
    })
    .unwrap();

    let g_size: i64 = fiedler_vector.iter().filter(|x| x > &&0.0).count() as i64;
    let other_size: i64 = arr_dim as i64 - g_size;
    return g_size * other_size;
}

fn part2(_contents: String) -> String {
    return "Christmas has been saved!".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 54);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2023".to_string();
	let day = "25".to_string();
	
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
        "\nPart 1:\nProduct of group sizes: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\n{}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}