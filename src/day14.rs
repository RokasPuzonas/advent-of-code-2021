use std::collections::HashMap;

pub struct InputData {
	polymer_template: String,
	rules: HashMap<[char; 2], char>,
}

pub fn parse_input(input: &str) -> InputData {
	let (polymer_template, rules_section) = input.split_once("\n\n").unwrap();
	let mut rules = HashMap::new();
	for line in rules_section.lines() {
		let (pattern, expansion) = line.split_once(" -> ").unwrap();
		rules.insert(
			[
				pattern.chars().nth(0).unwrap(),
				pattern.chars().nth(1).unwrap(),
			],
			expansion.chars().nth(0).unwrap(),
		);
	}
	return InputData {
		polymer_template: polymer_template.into(),
		rules,
	};
}

fn naive_expand_polymer(polymer: Vec<char>, rules: &HashMap<[char; 2], char>) -> Vec<char> {
	let mut new_polymer = Vec::new();
	for i in 0..polymer.len() - 1 {
		new_polymer.push(polymer[i]);
		let pair = [polymer[i], polymer[i + 1]];
		let rule = rules.get(&pair);
		if rule != None {
			new_polymer.push(*rule.unwrap());
		}
	}
	new_polymer.push(*polymer.last().unwrap());
	return new_polymer;
}

pub fn part1(input: &InputData) -> u32 {
	let mut polymer = input.polymer_template.chars().collect();
	for _ in 0..10 {
		polymer = naive_expand_polymer(polymer, &input.rules);
	}

	let mut element_amounts = HashMap::new();
	for c in polymer {
		let amount = element_amounts.entry(c).or_insert(0);
		*amount += 1;
	}

	let least_common_element = element_amounts
		.iter()
		.min_by(|a, b| a.1.cmp(b.1))
		.unwrap()
		.1;
	let most_common_element = element_amounts
		.iter()
		.max_by(|a, b| a.1.cmp(b.1))
		.unwrap()
		.1;
	return most_common_element - least_common_element;
}

fn expand_polymer(
	polymer_pairs: &HashMap<[char; 2], u64>,
	rules: &HashMap<[char; 2], char>,
) -> HashMap<[char; 2], u64> {
	let mut new_pairs = HashMap::new();
	for entry in polymer_pairs {
		let pair = entry.0;
		if rules.contains_key(pair) {
			let rule = *rules.get(pair).unwrap();

			let left_pair = [pair[0], rule];
			let left_entry = new_pairs.entry(left_pair).or_insert(0);
			*left_entry += entry.1;

			let right_pair = [rule, pair[1]];
			let right_entry = new_pairs.entry(right_pair).or_insert(0);
			*right_entry += entry.1;
		} else {
			let new_entry = new_pairs.entry(*entry.0).or_insert(0);
			*new_entry = *entry.1;
		}
	}
	return new_pairs;
}

pub fn part2(input: &InputData) -> u64 {
	let polymer_template = &input.polymer_template;
	let mut polymer_pairs = HashMap::new();
	for i in 0..polymer_template.len() - 1 {
		let pair = [
			polymer_template.chars().nth(i).unwrap(),
			polymer_template.chars().nth(i + 1).unwrap(),
		];
		let entry = polymer_pairs.entry(pair).or_insert(0);
		*entry += 1;
	}

	for _ in 0..40 {
		polymer_pairs = expand_polymer(&polymer_pairs, &input.rules);
	}

	let mut element_amounts = HashMap::new();
	for entry in polymer_pairs {
		for c in entry.0 {
			let amount = element_amounts.entry(c).or_insert(0);
			*amount += entry.1;
		}
	}

	let least_common_element = element_amounts
		.iter()
		.min_by(|a, b| a.1.cmp(b.1))
		.unwrap()
		.1;
	let most_common_element = element_amounts
		.iter()
		.max_by(|a, b| a.1.cmp(b.1))
		.unwrap()
		.1;
	return (most_common_element - least_common_element) / 2 + 1;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_example() {
		let polymer_template = String::from("NNCB");
		let rules = HashMap::from([
			(['C', 'H'], 'B'),
			(['H', 'H'], 'N'),
			(['C', 'B'], 'H'),
			(['N', 'H'], 'C'),
			(['H', 'B'], 'C'),
			(['H', 'C'], 'B'),
			(['H', 'N'], 'C'),
			(['N', 'N'], 'C'),
			(['B', 'H'], 'H'),
			(['N', 'C'], 'B'),
			(['N', 'B'], 'B'),
			(['B', 'N'], 'B'),
			(['B', 'B'], 'N'),
			(['B', 'C'], 'B'),
			(['C', 'C'], 'N'),
			(['C', 'N'], 'C'),
		]);
		let result = part1(&InputData {
			polymer_template,
			rules,
		});
		assert_eq!(result, 1588);
	}

	#[test]
	fn part2_example() {
		let polymer_template = String::from("NNCB");
		let rules = HashMap::from([
			(['C', 'H'], 'B'),
			(['H', 'H'], 'N'),
			(['C', 'B'], 'H'),
			(['N', 'H'], 'C'),
			(['H', 'B'], 'C'),
			(['H', 'C'], 'B'),
			(['H', 'N'], 'C'),
			(['N', 'N'], 'C'),
			(['B', 'H'], 'H'),
			(['N', 'C'], 'B'),
			(['N', 'B'], 'B'),
			(['B', 'N'], 'B'),
			(['B', 'B'], 'N'),
			(['B', 'C'], 'B'),
			(['C', 'C'], 'N'),
			(['C', 'N'], 'C'),
		]);
		let result = part2(&InputData {
			polymer_template,
			rules,
		});
		assert_eq!(result, 2188189693529);
	}
}
