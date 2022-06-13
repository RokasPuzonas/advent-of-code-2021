use std::collections::{HashMap, HashSet};

pub fn parse_input(input: &str) -> Vec<(String, String)> {
	let mut edges: Vec<(String, String)> = Vec::new();

	for line in input.lines() {
		let (from, to) = line.split_once('-').unwrap();
		edges.push((from.into(), to.into()));
	}

	return edges;
}

fn edges_to_map(edges: &Vec<(String, String)>) -> HashMap<String, Vec<String>> {
	let mut map = HashMap::new();

	for line in edges {
		map
			.entry(line.1.clone())
			.or_insert(Vec::new())
			.push(line.0.clone());

		map
			.entry(line.0.clone())
			.or_insert(Vec::new())
			.push(line.1.clone());
	}

	return map;
}

fn is_path_finished(path: &Vec<&str>) -> bool {
	return path.contains(&"end");
}

fn can_be_appended_part1(path: &Vec<&str>, node: &str) -> bool {
	node.to_uppercase() == node || !path.contains(&node)
}

pub fn part1(edges: &Vec<(String, String)>) -> usize {
	let map = edges_to_map(edges);

	let mut finished_paths: Vec<Vec<&str>> = Vec::new();

	let mut unfinished_paths: Vec<Vec<&str>> = Vec::new();
	unfinished_paths.push(vec!["start"]);
	while unfinished_paths.len() > 0 {
		let mut new_paths = Vec::new();

		for path in &mut unfinished_paths {
			for node in map.get(*path.last().unwrap()).unwrap() {
				if can_be_appended_part1(path, node) {
					let mut new_path = path.clone();
					new_path.push(node);

					if is_path_finished(&new_path) {
						finished_paths.push(new_path);
					} else {
						new_paths.push(new_path);
					}
				}
			}
		}

		unfinished_paths = new_paths;
	}

	return finished_paths.len();
}

fn can_be_appended_part2(path: &Vec<&str>, node: &str) -> bool {
	if node == "start" {
		return false;
	}
	if node == "end" {
		return true;
	}
	if node.to_uppercase() == node {
		return true;
	}

	// If all lowercase nodes only apear once we can be assure that any lowercase
	// node that will be added will be correct.
	let mut uniq = HashSet::new();
	if path
		.into_iter()
		.all(move |x| x.to_lowercase() != *x || uniq.insert(x))
	{
		return true;
	}

	return !path.contains(&node);
}

pub fn part2(edges: &Vec<(String, String)>) -> usize {
	let map = edges_to_map(edges);

	let mut finished_paths: Vec<Vec<&str>> = Vec::new();

	let mut unfinished_paths: Vec<Vec<&str>> = Vec::new();
	unfinished_paths.push(vec!["start"]);
	while unfinished_paths.len() > 0 {
		let mut new_paths = Vec::new();

		for path in &mut unfinished_paths {
			for node in map.get(*path.last().unwrap()).unwrap() {
				if can_be_appended_part2(path, node) {
					let mut new_path = path.clone();
					new_path.push(node);

					if is_path_finished(&new_path) {
						finished_paths.push(new_path);
					} else {
						new_paths.push(new_path);
					}
				}
			}
		}

		unfinished_paths = new_paths;
	}

	return finished_paths.len();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_example() {
		let cave_system = parse_input("start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end");
		let result = part1(&cave_system);
		assert_eq!(result, 10);
	}

	#[test]
	fn part1_larger_example() {
		let cave_system = parse_input(
			"dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc",
		);
		let result = part1(&cave_system);
		assert_eq!(result, 19);
	}

	#[test]
	fn part1_largest_example() {
		let cave_system = parse_input("fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW");
		let result = part1(&cave_system);
		assert_eq!(result, 226);
	}

	#[test]
	fn part2_example() {
		let cave_system = parse_input("start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end");
		let result = part2(&cave_system);
		assert_eq!(result, 36);
	}

	#[test]
	fn part2_larger_example() {
		let cave_system = parse_input(
			"dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc",
		);
		let result = part2(&cave_system);
		assert_eq!(result, 103);
	}

	#[test]
	fn part2_largest_example() {
		let cave_system = parse_input("fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW");
		let result = part2(&cave_system);
		assert_eq!(result, 3509);
	}
}
