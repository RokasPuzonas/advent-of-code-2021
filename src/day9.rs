use std::collections::HashSet;

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
	input
		.lines()
		.map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
		.collect()
}

fn find_low_points(grid: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
	let mut low_points = Vec::new();
	let height = grid.len();
	for i in 0..height {
		let width = grid[i].len();
		for j in 0..width {
			if (i == 0 || grid[i - 1][j] > grid[i][j])
				&& (i == height - 1 || grid[i + 1][j] > grid[i][j])
				&& (j == 0 || grid[i][j - 1] > grid[i][j])
				&& (j == width - 1 || grid[i][j + 1] > grid[i][j])
			{
				low_points.push((i, j));
			}
		}
	}
	return low_points;
}

pub fn part1(grid: Vec<Vec<u32>>) -> u32 {
	let mut sum = 0;
	for low_point in find_low_points(&grid) {
		let depth = grid[low_point.0][low_point.1];
		sum += depth + 1;
	}
	return sum;
}

fn find_basin_size(grid: &Vec<Vec<u32>>, location: (usize, usize)) -> u32 {
	let mut explored_spots = HashSet::new();
	let mut leaf_nodes = vec![location];
	let height = grid.len();
	let width = grid[0].len();

	while leaf_nodes.len() > 0 {
		let leaf_node = leaf_nodes.pop().unwrap();
		explored_spots.insert(leaf_node);

		let (i, j) = leaf_node;
		if i > 0 && !explored_spots.contains(&(i - 1, j)) && grid[i - 1][j] != 9 {
			leaf_nodes.push((i - 1, j));
		}
		if i < height - 1 && !explored_spots.contains(&(i + 1, j)) && grid[i + 1][j] != 9 {
			leaf_nodes.push((i + 1, j));
		}
		if j > 0 && !explored_spots.contains(&(i, j - 1)) && grid[i][j - 1] != 9 {
			leaf_nodes.push((i, j - 1));
		}
		if j < width - 1 && !explored_spots.contains(&(i, j + 1)) && grid[i][j + 1] != 9 {
			leaf_nodes.push((i, j + 1));
		}
	}

	return explored_spots.len() as u32;
}

fn find_basin_sizes(grid: &Vec<Vec<u32>>) -> Vec<u32> {
	let mut sizes = Vec::new();
	for low_point in find_low_points(&grid) {
		sizes.push(find_basin_size(grid, low_point))
	}
	return sizes;
}

pub fn part2(grid: Vec<Vec<u32>>) -> u32 {
	let mut basin_sizes = find_basin_sizes(&grid);
	basin_sizes.sort_by(|a, b| b.cmp(a));
	return basin_sizes[0] * basin_sizes[1] * basin_sizes[2];
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_example() {
		let input = vec![
			vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
			vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
			vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
			vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
			vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
		];
		let result = part1(input);
		assert_eq!(result, 15);
	}

	#[test]
	fn part2_example() {
		let input = vec![
			vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
			vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
			vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
			vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
			vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
		];
		let result = part2(input);
		assert_eq!(result, 1134);
	}
}
