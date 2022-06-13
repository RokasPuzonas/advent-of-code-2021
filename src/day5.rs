use std::{
	cmp::{max, min},
	num::ParseIntError,
};

type Grid = Vec<Vec<i32>>;

#[derive(Debug)]
pub struct Line {
	x1: i32,
	y1: i32,
	x2: i32,
	y2: i32,
}

#[derive(Debug)]
pub enum ParseLineError {
	ParseNumberError(ParseIntError),
	ParsePointsError,
	ParsePointError,
}

pub fn parse_point(input: &str) -> Result<(i32, i32), ParseLineError> {
	let mut parts = input.split(',');
	let x = parts
		.next()
		.ok_or(ParseLineError::ParsePointError)?
		.parse()
		.map_err(ParseLineError::ParseNumberError)?;
	let y = parts
		.next()
		.ok_or(ParseLineError::ParsePointError)?
		.parse()
		.map_err(ParseLineError::ParseNumberError)?;
	return Ok((x, y));
}

pub fn parse_input(input: &str) -> Result<Vec<Line>, ParseLineError> {
	let mut lines = Vec::new();
	for input_line in input.lines() {
		let mut parts = input_line.split(" -> ");

		let point1 = parts.next().ok_or(ParseLineError::ParsePointsError)?;
		let (x1, y1) = parse_point(point1)?;

		let point2 = parts.next().ok_or(ParseLineError::ParsePointsError)?;
		let (x2, y2) = parse_point(point2)?;

		lines.push(Line { x1, y1, x2, y2 })
	}
	return Ok(lines);
}

fn determine_bounds(lines: &[Line]) -> (i32, i32, i32, i32) {
	let mut x1 = i32::MAX;
	let mut y1 = i32::MAX;
	let mut x2 = i32::MIN;
	let mut y2 = i32::MIN;
	for line in lines {
		x1 = min(x1, min(line.x1, line.x2));
		y1 = min(y1, min(line.y1, line.y2));
		x2 = max(x2, max(line.x1, line.x2));
		y2 = max(y2, max(line.y1, line.y2));
	}
	return (x1, y1, x2, y2);
}

fn new_grid(width: usize, height: usize) -> Grid {
	let mut grid = Vec::new();
	for _ in 0..height {
		grid.push(vec![0; width]);
	}
	grid
}

fn sign(x: i32) -> i32 {
	if x > 0 {
		1
	} else if x < 0 {
		-1
	} else {
		0
	}
}

fn mark_line(grid: &mut Grid, line: &Line, ox: i32, oy: i32) {
	let dx = sign(line.x2 - line.x1);
	let dy = sign(line.y2 - line.y1);
	let mut x = line.x1;
	let mut y = line.y1;
	while x != line.x2 || y != line.y2 {
		grid[(y - oy) as usize][(x - ox) as usize] += 1;
		if x != line.x2 {
			x += dx;
		}
		if y != line.y2 {
			y += dy;
		}
	}
	grid[(y - oy) as usize][(x - ox) as usize] += 1;
}

fn count_dangerous_areas(grid: &Grid) -> u32 {
	let mut count = 0;
	for row in grid {
		for point in row {
			if *point > 1 {
				count += 1;
			}
		}
	}
	count
}

pub fn part1(lines: &[Line]) -> u32 {
	let bounds = determine_bounds(lines);
	let width = (bounds.2 - bounds.0 + 1) as usize;
	let height = (bounds.3 - bounds.1 + 1) as usize;
	let mut grid = new_grid(width, height);

	for line in lines {
		if (line.x1 == line.x2) || (line.y1 == line.y2) {
			mark_line(&mut grid, line, bounds.0, bounds.1);
		}
	}

	count_dangerous_areas(&grid)
}

pub fn part2(lines: &[Line]) -> u32 {
	let bounds = determine_bounds(lines);
	let width = (bounds.2 - bounds.0 + 1) as usize;
	let height = (bounds.3 - bounds.1 + 1) as usize;
	let mut grid = new_grid(width, height);

	for line in lines {
		mark_line(&mut grid, line, bounds.0, bounds.1);
	}

	count_dangerous_areas(&grid)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_example() {
		let input = [
			Line {
				x1: 0,
				y1: 9,
				x2: 5,
				y2: 9,
			},
			Line {
				x1: 8,
				y1: 0,
				x2: 0,
				y2: 8,
			},
			Line {
				x1: 9,
				y1: 4,
				x2: 3,
				y2: 4,
			},
			Line {
				x1: 2,
				y1: 2,
				x2: 2,
				y2: 1,
			},
			Line {
				x1: 7,
				y1: 0,
				x2: 7,
				y2: 4,
			},
			Line {
				x1: 6,
				y1: 4,
				x2: 2,
				y2: 0,
			},
			Line {
				x1: 0,
				y1: 9,
				x2: 2,
				y2: 9,
			},
			Line {
				x1: 3,
				y1: 4,
				x2: 1,
				y2: 4,
			},
			Line {
				x1: 0,
				y1: 0,
				x2: 8,
				y2: 8,
			},
			Line {
				x1: 5,
				y1: 5,
				x2: 8,
				y2: 2,
			},
		];
		let result = part1(&input);
		assert_eq!(result, 5);
	}

	#[test]
	fn part2_example() {
		let input = [
			Line {
				x1: 0,
				y1: 9,
				x2: 5,
				y2: 9,
			},
			Line {
				x1: 8,
				y1: 0,
				x2: 0,
				y2: 8,
			},
			Line {
				x1: 9,
				y1: 4,
				x2: 3,
				y2: 4,
			},
			Line {
				x1: 2,
				y1: 2,
				x2: 2,
				y2: 1,
			},
			Line {
				x1: 7,
				y1: 0,
				x2: 7,
				y2: 4,
			},
			Line {
				x1: 6,
				y1: 4,
				x2: 2,
				y2: 0,
			},
			Line {
				x1: 0,
				y1: 9,
				x2: 2,
				y2: 9,
			},
			Line {
				x1: 3,
				y1: 4,
				x2: 1,
				y2: 4,
			},
			Line {
				x1: 0,
				y1: 0,
				x2: 8,
				y2: 8,
			},
			Line {
				x1: 5,
				y1: 5,
				x2: 8,
				y2: 2,
			},
		];
		let result = part2(&input);
		assert_eq!(result, 12);
	}
}
