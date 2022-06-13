use std::num::ParseIntError;

pub fn parse_input(input: &str) -> Result<Vec<i32>, ParseIntError> {
	input
		.trim_end()
		.split_terminator(',')
		.map(|s| s.parse())
		.collect()
}

fn abs(x: i32) -> u32 {
	if x < 0 {
		-x as u32
	} else {
		x as u32
	}
}

fn calculate_total_cost_to1(crabs: &[i32], target: i32) -> u32 {
	let mut sum = 0;
	for crab in crabs {
		sum += abs(crab - target);
	}
	return sum;
}

pub fn part1(crabs: &[i32]) -> u32 {
	let mut best_cost = calculate_total_cost_to1(crabs, crabs[0]);

	for position in crabs.iter().skip(1) {
		let cost = calculate_total_cost_to1(crabs, *position);
		if cost < best_cost {
			best_cost = cost;
		}
	}

	best_cost
}

fn calculate_total_cost_to2(crabs: &[i32], target: i32) -> u32 {
	let mut sum = 0;
	for crab in crabs {
		let distance = abs(crab - target);
		for i in 1..=distance {
			sum += i;
		}
	}
	return sum;
}

fn calculate_average(arr: &[i32]) -> f32 {
	let mut sum = 0.0;
	for a in arr {
		sum += *a as f32;
	}
	return sum / arr.len() as f32;
}

pub fn part2(crabs: &[i32]) -> u32 {
	let average_position = calculate_average(crabs).round() as i32;
	let mut best_cost = calculate_total_cost_to2(crabs, average_position);

	for position in average_position - 5..=average_position + 5 {
		let cost = calculate_total_cost_to2(crabs, position);
		if cost < best_cost {
			best_cost = cost;
		}
	}

	best_cost
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_example() {
		let input = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
		let result = part1(&input);
		assert_eq!(result, 37);
	}

	#[test]
	fn part2_example() {
		let input = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
		let result = part2(&input);
		assert_eq!(result, 168);
	}
}
