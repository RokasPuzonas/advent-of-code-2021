use core::fmt;
use std::cmp::Reverse;

use priority_queue::PriorityQueue;

const HALLWAY_POINTS: &[Point] = &[Point(1, 1), Point(2, 1), Point(4, 1), Point(6, 1), Point(8, 1), Point(10, 1), Point(11, 1)];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Amphipod {
	A, B, C, D
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point(u32, u32);

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				write!(f, "Point({},{})", self.0, self.1)?;
				Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Map {
	room_size: u32,
	amphipod_positions: Vec<Point>,
	amphipod_types: Vec<Amphipod>
}

pub fn parse_input(input: &str) -> Map {
	let mut positions = vec![];
	let mut types = vec![];
	for (y, line) in input.lines().enumerate() {
		for (x, c) in line.chars().enumerate() {
			let amphipod = match c {
				'A' => Amphipod::A,
				'B' => Amphipod::B,
				'C' => Amphipod::C,
				'D' => Amphipod::D,
				_ => continue
			};

			positions.push(Point(x as u32, y as u32));
			types.push(amphipod);
		}
	}

	let room_size = (input.lines().count()-3) as u32;

	Map {
		room_size,
		amphipod_types: types,
		amphipod_positions: positions
	}
}

fn is_in_hallway(point: &Point) -> bool {
	point.1 == 1
}

fn get_home_column(amphipod: &Amphipod) -> u32 {
	match amphipod {
    Amphipod::A => 3,
    Amphipod::B => 5,
    Amphipod::C => 7,
    Amphipod::D => 9,
	}
}

fn get_amphipod_cost(amphipod: &Amphipod) -> u32 {
	match amphipod {
    Amphipod::A => 1,
    Amphipod::B => 10,
    Amphipod::C => 100,
    Amphipod::D => 1000
	}
}

#[allow(dead_code)]
fn dbg_map(map: &Map, positions: &[Point]) {
	fn print_amphipod(map: &Map, positions: &[Point], x: u32, y: u32) {
		for (i, Point(ix, iy)) in positions.iter().enumerate() {
			if *ix == x && *iy == y {
				match map.amphipod_types[i] {
					Amphipod::A => print!("A"),
					Amphipod::B => print!("B"),
					Amphipod::C => print!("C"),
					Amphipod::D => print!("D"),
				}
				return;
			}
		}

		print!(".");
	}

	println!("#############");
	print!("#");
	for x in 1..=11 {
		print_amphipod(map, positions, x, 1);
	}
	println!("#");

	print!("###");
	print_amphipod(map, positions, 3, 2);
	print!("#");
	print_amphipod(map, positions, 5, 2);
	print!("#");
	print_amphipod(map, positions, 7, 2);
	print!("#");
	print_amphipod(map, positions, 9, 2);
	println!("###");

	for y in 3..2+map.room_size {
		print!("  #");
		print_amphipod(map, positions, 3, y);
		print!("#");
		print_amphipod(map, positions, 5, y);
		print!("#");
		print_amphipod(map, positions, 7, y);
		print!("#");
		print_amphipod(map, positions, 9, y);
		println!("#  ");
	}

	println!("  #########  ");
}

fn is_solved(map: &Map, positions: &[Point]) -> bool {
	for (i, position) in positions.iter().enumerate() {
		let home_column = get_home_column(&map.amphipod_types[i]);
		if home_column != position.0 {
			return false;
		}
	}
	true
}

fn solve(map: &Map) -> u32 {
	if is_solved(map, &map.amphipod_positions) { return 0; }

	fn push_next_state(states: &mut PriorityQueue<Vec<Point>, Reverse<u32>>, next_state: Vec<Point>, cost: u32) {
		if let Some((_, Reverse(v))) = states.get(&next_state) {
			if cost < *v {
				states.push(next_state, Reverse(cost));
			}
		} else {
			states.push(next_state, Reverse(cost));
		}
	}

	fn has_point(points: &[Point], x: u32, y: u32) -> bool {
		for p in points {
			if p.0 == x && p.1 == y {
				return true;
			}
		}
		false
	}

	fn get_top_free_point(map: &Map, state: &[Point], column: u32) -> Point {
		for y in (2..(2+map.room_size)).rev() {
			if !has_point(state, column, y) {
				return Point(column, y);
			}
		}
		panic!("this should never happen")
	}

	let mut states = PriorityQueue::new();
	states.push(map.amphipod_positions.clone(), Reverse(0));
	while !states.is_empty() {
		let (state, Reverse(state_cost)) = states.pop().unwrap();

		if is_solved(map, &state) {
			return state_cost;
		}

		'outer: for (i, point) in state.iter().enumerate() {
			let home_column = get_home_column(&map.amphipod_types[i]);
			let step_cost = get_amphipod_cost(&map.amphipod_types[i]);

			if is_in_hallway(point) {
				for y in (2..(2+map.room_size)).rev() {
					let index = state.iter().position(|p| p.0 == home_column && p.1 == y);
					if let Some(index) = index {
						if map.amphipod_types[index] != map.amphipod_types[i] { continue 'outer }
					}
				}

				let x_range = {
					if point.0 > home_column {
						home_column..=(point.0-1)
					} else {
						(point.0+1)..=home_column
					}
				};
				for x in x_range {
					if has_point(&state, x, 1) { continue 'outer }
				}

				let top_free_point = get_top_free_point(map, &state, home_column);
				let mut next_state = state.clone();
				next_state[i] = top_free_point;
				let cost = state_cost + ((top_free_point.1 - 1) + (home_column as i32 - point.0 as i32).abs() as u32) * step_cost;
				push_next_state(&mut states, next_state, cost);
			} else {
				// Check if there are not amphipod above
				for y in 2..point.1 {
					if has_point(&state, point.0, y) { continue 'outer }
				}

				// Check if amphipod is block anything below it
				if home_column == point.0 {
					let mut needs_moving = false;
					for y in (point.1+1)..=(2+map.room_size) {
						let index = state.iter().position(|p| p.0 == point.0 && p.1 == y);
						if let Some(index) = index {
							if map.amphipod_types[index] != map.amphipod_types[i] {
								needs_moving = true;
								break;
							}
						}
					}

					if !needs_moving {
						continue 'outer;
					}
				}

				'inner: for hallway_point in HALLWAY_POINTS {
					if !state.contains(hallway_point) {
						let from_x = point.0.min(hallway_point.0);
						let to_x = point.0.max(hallway_point.0);
						for x in from_x..to_x {
							if has_point(&state, x, 1) { continue 'inner }
						}

						let mut next_state = state.clone();
						next_state[i] = *hallway_point;
						let cost = state_cost + ((point.1 - 1) + (to_x - from_x)) * step_cost;
						push_next_state(&mut states, next_state, cost);
					}
				}
			}
		}
	}

	panic!("how did we get here?");
}

pub fn part1(map: Map) -> u32 {
	solve(&map)
}

pub fn part2(mut map: Map) -> u32 {
	map.room_size = 4;
	for position in [Point(3, 3), Point(5,3), Point(7,3), Point(9,3)] {
		let index = map.amphipod_positions.iter().position(|p| *p == position);
		if let Some(index) = index {
			map.amphipod_positions[index] = Point(position.0, 5);
		}
	}

	let new_amphipods = [
		(Point(3, 3), Amphipod::D),
		(Point(3, 4), Amphipod::D),
		(Point(5, 3), Amphipod::C),
		(Point(5, 4), Amphipod::B),
		(Point(7, 3), Amphipod::B),
		(Point(7, 4), Amphipod::A),
		(Point(9, 3), Amphipod::A),
		(Point(9, 4), Amphipod::C),
	];
	for (position, r#type) in new_amphipods {
		map.amphipod_positions.push(position);
		map.amphipod_types.push(r#type);
	}

	solve(&map)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_example() {
 		let map = parse_input(&[
			"#############",
			"#...........#",
			"###B#C#B#D###",
			"  #A#D#C#A#  ",
			"  #########  "
		].join("\n"));
		let result = part1(map);
		assert_eq!(result, 12521);
	}

	#[test]
	fn my_input() {
	 		let map = parse_input(&[
			"#############",
			"#...........#",
			"###C#B#D#A###",
			"  #B#D#A#C#  ",
			"  #########  "
		].join("\n"));
		let result = part1(map);
		assert_eq!(result, 13520);
	}

	#[test]
	fn part2_example() {
	 		let map = parse_input(&[
			"#############",
			"#...........#",
			"###B#C#B#D###",
			"  #A#D#C#A#  ",
			"  #########  "
		].join("\n"));
		let result = part2(map);
		assert_eq!(result, 44169);
	}
}
