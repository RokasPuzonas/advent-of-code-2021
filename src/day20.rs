use std::{fmt::Display, ops::Range};

#[derive(Clone, Debug)]
pub struct Image {
	width: usize,
	height: usize,
	data: Vec<bool>,
	default_value: bool,
	offset_x: i32,
	offset_y: i32,
}

impl Image {
	fn new(width: usize, height: usize) -> Image {
		let data = vec![false; width * height];
		Image {
			width,
			height,
			data,
			default_value: false,
			offset_x: 0,
			offset_y: 0,
		}
	}

	fn get(&self, x: i32, y: i32) -> bool {
		if self.in_bounds(x, y) {
			let index = (y + self.offset_y) * self.width as i32 + (x + self.offset_x);
			*self.data.get(index as usize).unwrap_or(&self.default_value)
		} else {
			self.default_value
		}
	}

	fn set(&mut self, x: i32, y: i32, value: bool) {
		// Make image larger if it is too small
		let (left, top, right, bottom) = self.bounds();
		if !(left <= x && x < right && top <= y && y < bottom) {
			if x < left {
				self.offset_x = -x;
				self.width = self.width + (-x + left) as usize;
			} else if x >= right {
				self.width = self.width + 1 + (x - right) as usize;
			}

			if y < top {
				self.offset_y = -y;
				self.height = self.height + (-y + top) as usize;
			} else if y >= bottom {
				self.height = self.height + 1 + (y - bottom) as usize;
			}

			let mut new_data = Vec::new();
			for y in self.y_range(0) {
				for x in self.x_range(0) {
					new_data.push(self.get(x, y));
				}
			}
			self.data = new_data;
		}

		let index = ((y + self.offset_y) * self.width as i32 + (x + self.offset_x)) as usize;
		self.data[index] = value;
	}

	fn in_bounds(&self, x: i32, y: i32) -> bool {
		let (left, top, right, bottom) = self.bounds();
		left <= x && x < right && top <= y && y < bottom
	}

	fn count(&self, value: bool) -> usize {
		if self.default_value == value {
			usize::MAX
		} else {
			self.data.iter().filter(|x| **x == value).count()
		}
	}

	fn x_range(&self, padding: u32) -> Range<i32> {
		let padding = padding as i32;
		Range {
			start: -self.offset_x - padding,
			end: self.width as i32 - self.offset_x + padding,
		}
	}

	fn y_range(&self, padding: u32) -> Range<i32> {
		let padding = padding as i32;
		Range {
			start: -self.offset_y - padding,
			end: self.height as i32 - self.offset_y + padding,
		}
	}

	// (i32, i32, i32, i32) => (left, top, right, bottom)
	fn bounds(&self) -> (i32, i32, i32, i32) {
		return (
			-self.offset_x,
			-self.offset_y,
			self.width as i32 - self.offset_x,
			self.height as i32 - self.offset_y,
		);
	}
}

impl Display for Image {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut grid = String::new();
		for y in self.y_range(1) {
			for x in self.x_range(1) {
				let symbol = if self.get(x, y) { '#' } else { '.' };
				grid.push(symbol);
			}
			grid.push('\n');
		}
		write!(f, "{}", grid)
	}
}

fn parse_image(input: &str) -> Image {
	let mut image = Image::new(3, 3);
	let mut y = 0;
	for line in input.lines() {
		let mut x = 0;
		for c in line.chars() {
			image.set(x, y, c == '#');
			x += 1;
		}
		y += 1
	}
	return image;
}

fn parse_enchancer(input: &str) -> [bool; 512] {
	let mut enhancer = [false; 512];
	let mut i = 0;
	for c in input.chars() {
		enhancer[i] = c == '#';
		i += 1;
	}
	return enhancer;
}

pub fn parse_input(input: &str) -> ([bool; 512], Image) {
	let (section1, section2) = input.split_once("\n\n").unwrap();
	return (parse_enchancer(section1), parse_image(section2));
}

fn lookup_enhancer(x: i32, y: i32, image: &Image, enhancer: &[bool; 512]) -> bool {
	let pixel_offsets = [
		(-1, -1),
		(0, -1),
		(1, -1),
		(-1, 0),
		(0, 0),
		(1, 0),
		(-1, 1),
		(0, 1),
		(1, 1),
	];

	let mut lookup_index = 0;
	for i in 0..9 {
		let (ox, oy) = pixel_offsets[i];
		let pixel = image.get(x + ox, y + oy);
		if pixel {
			lookup_index += 2usize.pow((8 - i) as u32)
		}
	}

	enhancer[lookup_index]
}

fn enhance(image: &Image, enhancer: &[bool; 512]) -> Image {
	let mut enhanced = image.clone();
	for y in image.y_range(1) {
		for x in image.x_range(1) {
			enhanced.set(x, y, lookup_enhancer(x, y, image, enhancer));
		}
	}

	if image.default_value {
		enhanced.default_value = enhancer[511];
	} else {
		enhanced.default_value = enhancer[0];
	}

	enhanced
}

pub fn part1(data: &([bool; 512], Image)) -> usize {
	let (enhancer, image) = data;
	let mut enhanced_image = enhance(&image, enhancer);
	enhanced_image = enhance(&enhanced_image, enhancer);
	enhanced_image.count(true)
}

pub fn part2(data: &([bool; 512], Image)) -> usize {
	let (enhancer, image) = data;
	let mut enhanced_image = enhance(&image, enhancer);
	for _ in 0..49 {
		enhanced_image = enhance(&enhanced_image, enhancer);
	}
	enhanced_image.count(true)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_example() {
		let input = parse_input("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###");
		let result = part1(&input);
		assert_eq!(result, 35);
	}

	#[test]
	fn part2_example() {
		let input = parse_input("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###");
		let result = part2(&input);
		assert_eq!(result, 3351);
	}
}
