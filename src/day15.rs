use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Grid {
	rows: u32,
	cols: u32,
	data: Vec<u32>,
	scale: u32,
}

impl Grid {
	fn new(rows: u32, cols: u32, data: Vec<u32>) -> Grid {
		return Grid {
			rows,
			cols,
			data,
			scale: 1,
		};
	}

	fn get(&self, point: &(u32, u32)) -> u32 {
		let row = point.0 % self.rows;
		let col = point.1 % self.cols;
		let value = self.data[(row * self.cols + col) as usize];
		return (value + point.0 / self.rows + point.1 / self.cols - 1) % 9 + 1;
	}

	fn width(&self) -> u32 {
		self.cols * self.scale
	}

	fn height(&self) -> u32 {
		self.rows * self.scale
	}

	fn within_bounds(&self, point: &(i32, i32)) -> bool {
		point.0 >= 0 && point.1 >= 0 && point.0 < self.height() as i32 && point.1 < self.width() as i32
	}
}

pub fn parse_input(input: &str) -> Grid {
	let mut data = Vec::new();
	let mut rows = 0;
	let mut cols = 0;
	for line in input.lines() {
		rows += 1;
		for c in line.chars() {
			if rows == 1 {
				cols += 1
			}
			data.push(c.to_digit(10).unwrap());
		}
	}
	return Grid::new(rows, cols, data);
}

fn find_neighbours(pos: &(u32, u32), grid: &Grid, offsets: &[(i32, i32)]) -> Vec<(u32, u32)> {
	let mut neighbours = Vec::new();
	for offset in offsets {
		let row = pos.0 as i32 + offset.0;
		let col = pos.1 as i32 + offset.1;
		if grid.within_bounds(&(row, col)) {
			neighbours.push((row as u32, col as u32));
		}
	}
	return neighbours;
}

fn find_shortest_path_cost(grid: &Grid) -> u32 {
	let mut total_costs: HashMap<(u32, u32), u32> = HashMap::new();
	let mut min_pq: PriorityQueue<(u32, u32), i32> = PriorityQueue::new();
	let mut visited: HashSet<(u32, u32)> = HashSet::new();
	let neighbour_offsets = [(0, 1), (0, -1), (1, 0), (-1, 0)];
	min_pq.push((0, 0), 0);
	total_costs.insert((0, 0), 0);

	while !min_pq.is_empty() {
		let new_smallest = min_pq.pop().unwrap().0;
		visited.insert(new_smallest);

		for neighbour in find_neighbours(&new_smallest, grid, &neighbour_offsets) {
			if visited.contains(&neighbour) {
				continue;
			}

			let alt_distance = grid.get(&neighbour);
			let alt_path = total_costs.get(&new_smallest).unwrap_or(&u32::MAX) + alt_distance;
			if alt_path < *total_costs.get(&neighbour).unwrap_or(&u32::MAX) {
				total_costs.insert(neighbour, alt_path);
				min_pq.push_decrease(neighbour, -(alt_path as i32));
			}
		}
	}

	return *total_costs
		.get(&(grid.height() - 1, grid.width() - 1))
		.unwrap();
}

pub fn part1(grid: &Grid) -> u32 {
	find_shortest_path_cost(grid)
}

pub fn part2(grid: &mut Grid) -> u32 {
	grid.scale = 5;
	find_shortest_path_cost(grid)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_example() {
		let grid = parse_input(
			"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581",
		);
		let result = part1(&grid);
		assert_eq!(result, 40);
	}

	#[test]
	fn part2_example() {
		let mut grid = parse_input(
			"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581",
		);
		let result = part2(&mut grid);
		assert_eq!(result, 315);
	}
}
