use std::{collections::{HashSet, HashMap}, ops::{Add, Sub, Neg}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point(i32, i32, i32);

fn parse_point(line: &str) -> Point {
	let nums: Vec<&str> = line.splitn(3, ',').collect();
	let x = nums[0].parse().unwrap();
	let y = nums[1].parse().unwrap();
	let z = nums[2].parse().unwrap();
	Point(x, y, z)
}

fn parse_scanner(input: &str) -> Vec<Point> {
	let mut beacons = Vec::new();
	for line in input.lines().skip(1) {
		beacons.push(parse_point(line));
	}
	beacons
}

pub fn parse_input(input: &str) -> Vec<Vec<Point>> {
	let mut scanners = Vec::new();
	for scanner_section in input.split("\n\n") {
		scanners.push(parse_scanner(scanner_section));
	}
	scanners
}

impl Add for Point {
		type Output = Self;

		fn add(self, rhs: Self) -> Self::Output {
				Point(
						self.0 + rhs.0,
						self.1 + rhs.1,
						self.2 + rhs.2
				)
		}
}

impl Sub for Point {
		type Output = Self;

		fn sub(self, rhs: Self) -> Self::Output {
				Point(
						self.0 - rhs.0,
						self.1 - rhs.1,
						self.2 - rhs.2
				)
		}
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
				Point(-self.0, -self.1, -self.2)
    }
}

fn calc_diff(a: &Point, b: &Point) -> Point {
	let x = (a.0 - b.0).abs();
	let y = (a.1 - b.1).abs();
	let z = (a.2 - b.2).abs();

	let lowest = x.min(y).min(z);
	let highest = x.max(y).max(z);
	let middle = x + y + z - lowest - highest;

	Point(lowest, middle, highest)
}

fn get_point_pairs(points: &[Point]) -> Vec<(Point, Point)> {
	points.iter()
		.enumerate()
		.flat_map(|(i, p1)| points.iter().skip(i + 1).map(move |p2| (*p1, *p2)))
		.collect()
}

fn get_point_diffs(a: &[Point]) -> HashSet<Point> {
	get_point_pairs(a).iter()
		.map(|(p1, p2)| calc_diff(p1, p2))
		.collect()
}

fn do_scanners_possibly_overlap(a: &[Point], b: &[Point]) -> bool {
	let a_diffs = get_point_diffs(a);
	let b_diffs = get_point_diffs(b);

	a_diffs.intersection(&b_diffs).count() >= 66
}

fn find_possibly_overlapping_scanners(scanners: &[Vec<Point>]) -> Vec<(usize, usize)> {
	let mut pairs = vec![];

	for (i, scanner1) in scanners.iter().enumerate() {
		for (j, scanner2) in scanners.iter().enumerate().skip(i + 1) {
			if do_scanners_possibly_overlap(scanner1, scanner2) {
				pairs.push((i, j));
			}
		}
	}

	pairs
}

fn calc_rotations(p: Point) -> Vec<Point> {
	// https://i.imgur.com/Ff1vGT9.png
	let Point(x, y, z) = p;
	vec![
		Point(x ,y ,z), Point(x ,z,-y), Point(x ,-y,-z), Point(x ,-z,y  ),
		Point(-x,-y,z), Point(-x,z,y ), Point(-x,y ,-z), Point(-x,-z,-y ),
		Point(y ,z ,x), Point(y ,x,-z), Point(y ,-z,-x), Point(y ,-x,z  ),
		Point(-y,-z,x), Point(-y,x,z ), Point(-y,z ,-x), Point(-y,-x,-z ),
		Point(z ,x ,y), Point(z ,y,-x), Point(z ,-x,-y), Point(z ,-y,x  ),
		Point(-z,-x,y), Point(-z,y,x ), Point(-z,x ,-y), Point(-z,-y,-x)
	]
}

fn calc_rotation(p: Point, rotation: usize) -> Point {
	calc_rotations(p)[rotation]
}

fn combine_rotations(r1: usize, r2: usize) -> usize {
	let target_point = calc_rotation(calc_rotation(Point(1, 2, 3), r1), r2);
	for (i, rotated) in calc_rotations(Point(1, 2, 3)).iter().enumerate() {
		if *rotated == target_point {
			return i;
		}
	}
	panic!()
}

fn invert_rotation(r: usize) -> usize {
	let target_point = calc_rotation(Point(1, 2, 3), r);
	for (i, rotated) in calc_rotations(target_point).iter().enumerate() {
		if *rotated == Point(1, 2, 3) {
			return i;
		}
	}
	panic!()
}

fn find_possible_rotations(dir1: Point, dir2: Point) -> Vec<usize> {
	let mut possible_rotations = vec![];

	let rotated_dirs2 = calc_rotations(dir2);
	for (i, rotated_dir2) in rotated_dirs2.iter().enumerate() {
		if *rotated_dir2 == dir1 {
			possible_rotations.push(i);
			break;
		}
	}

	for (i, rotated_dir2) in rotated_dirs2.iter().enumerate() {
		if *rotated_dir2 == -dir1 {
			possible_rotations.push(i);
			break;
		}
	}

	possible_rotations
}

fn apply_transformation(points: &[Point], translation: Point, rotation: usize) -> Vec<Point> {
	points.iter()
		.map(|p| calc_rotation(*p, rotation) + translation)
		.collect()
}

fn find_transformation(scanner1: &[Point], scanner2: &[Point]) -> Option<(Point, usize)> {
	let point_pairs1 = get_point_pairs(scanner1);
	let point_pairs2 = get_point_pairs(scanner2);

	let scanner1_set: HashSet<_> = scanner1.iter().copied().collect();

	for pair1 in &point_pairs1 {
		for pair2 in &point_pairs2 {
			if calc_diff(&pair1.0, &pair1.1) != calc_diff(&pair2.0, &pair2.1) {
				continue;
			}
			let dir1 = pair1.0 - pair1.1;
			let dir2 = pair2.0 - pair2.1;

			for rotation in find_possible_rotations(dir1, dir2) {
				let translation = pair1.0 - calc_rotation(pair2.0, rotation);
				let transformed_scanner2: HashSet<_> = scanner2.iter()
					.map(|p| calc_rotation(*p, rotation) + translation)
					.collect();

				if scanner1_set.intersection(&transformed_scanner2).count() >= 12 {
					return Some((translation, rotation));
				}
			}
		}
	}

	None
}

fn find_canonical_transformations(scanners: &[Vec<Point>]) -> Vec<(Point, usize)> {
	let mut canonical = vec![];
	for _ in 0..scanners.len() {
		canonical.push((Point(0, 0, 0), 0))
	}

	let mut transforms = HashMap::new();
	for (scanner1, scanner2) in find_possibly_overlapping_scanners(scanners) {
		let (translation, rotation) = find_transformation(&scanners[scanner1], &scanners[scanner2]).unwrap();
		let inv_rotation = invert_rotation(rotation);

		transforms.entry(scanner1)
			.or_insert_with(Vec::new)
			.push((scanner2, translation, rotation));

		transforms.entry(scanner2)
			.or_insert_with(Vec::new)
			.push((scanner1, calc_rotation(-translation, inv_rotation), inv_rotation));
	}

	let mut stack = vec![(0, Point(0, 0, 0), 0)];
	let mut visited = HashSet::new();
	while !stack.is_empty() {
		let (id, translation, rotation) = stack.pop().unwrap();

		if visited.contains(&id) { continue; }
		visited.insert(id);

		for nbr in transforms.get(&id).unwrap() {
			if visited.contains(&nbr.0) { continue; }

			let new_translation = calc_rotation(nbr.1, rotation) + translation;
			let new_rotation = combine_rotations(nbr.2, rotation);
			stack.push((nbr.0, new_translation, new_rotation));
			canonical[nbr.0] = (new_translation, new_rotation);
		}
	}

	// dbg!(&canonical);

	canonical
}

pub fn part1(scanners: &[Vec<Point>]) -> u32 {
	let transforms = find_canonical_transformations(scanners);
	let mut beacons = HashSet::new();

	for (i, points) in scanners.iter().enumerate() {
		let (translation, rotation) = transforms[i];
		for point in apply_transformation(points, translation, rotation) {
			beacons.insert(point);
		}
	}

	beacons.len() as u32
}

pub fn part2(scanners: &[Vec<Point>]) -> i32 {
	let transforms = find_canonical_transformations(scanners);

	let mut max_distance = 0;
	for (i, transform1) in transforms.iter().enumerate() {
		for transform2 in transforms.iter().skip(i+1) {
			let diff = transform1.0 - transform2.0;
			let distance = diff.0 + diff.1 + diff.2;
			max_distance = max_distance.max(distance);
		}
	}

	max_distance
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[rustfmt::skip]
	fn part1_example() {
		let scanners = vec![
			vec![Point(404, -588, -901), Point(528, -643, 409), Point(-838, 591, 734), Point(390, -675, -793), Point(-537, -823, -458), Point(-485, -357, 347), Point(-345, -311, 381), Point(-661, -816, -575), Point(-876, 649, 763), Point(-618, -824, -621), Point(553, 345, -567), Point(474, 580, 667), Point(-447, -329, 318), Point(-584, 868, -557), Point(544, -627, -890), Point(564, 392, -477), Point(455, 729, 728), Point(-892, 524, 684), Point(-689, 845, -530), Point(423, -701, 434), Point(7, -33, -71), Point(630, 319, -379), Point(443, 580, 662), Point(-789, 900, -551), Point(459, -707, 401)],
			vec![Point(686, 422, 578), Point(605, 423, 415), Point(515, 917, -361), Point(-336, 658, 858), Point(95, 138, 22), Point(-476, 619, 847), Point(-340, -569, -846), Point(567, -361, 727), Point(-460, 603, -452), Point(669, -402, 600), Point(729, 430, 532), Point(-500, -761, 534), Point(-322, 571, 750), Point(-466, -666, -811), Point(-429, -592, 574), Point(-355, 545, -477), Point(703, -491, -529), Point(-328, -685, 520), Point(413, 935, -424), Point(-391, 539, -444), Point(586, -435, 557), Point(-364, -763, -893), Point(807, -499, -711), Point(755, -354, -619), Point(553, 889, -390)],
			vec![Point(649, 640, 665), Point(682, -795, 504), Point(-784, 533, -524), Point(-644, 584, -595), Point(-588, -843, 648), Point(-30, 6, 44), Point(-674, 560, 763), Point(500, 723, -460), Point(609, 671, -379), Point(-555, -800, 653), Point(-675, -892, -343), Point(697, -426, -610), Point(578, 704, 681), Point(493, 664, -388), Point(-671, -858, 530), Point(-667, 343, 800), Point(571, -461, -707), Point(-138, -166, 112), Point(-889, 563, -600), Point(646, -828, 498), Point(640, 759, 510), Point(-630, 509, 768), Point(-681, -892, -333), Point(673, -379, -804), Point(-742, -814, -386), Point(577, -820, 562)],
			vec![Point(-589, 542, 597), Point(605, -692, 669), Point(-500, 565, -823), Point(-660, 373, 557), Point(-458, -679, -417), Point(-488, 449, 543), Point(-626, 468, -788), Point(338, -750, -386), Point(528, -832, -391), Point(562, -778, 733), Point(-938, -730, 414), Point(543, 643, -506), Point(-524, 371, -870), Point(407, 773, 750), Point(-104, 29, 83), Point(378, -903, -323), Point(-778, -728, 485), Point(426, 699, 580), Point(-438, -605, -362), Point(-469, -447, -387), Point(509, 732, 623), Point(647, 635, -688), Point(-868, -804, 481), Point(614, -800, 639), Point(595, 780, -596)],
			vec![Point(727, 592, 562), Point(-293, -554, 779), Point(441, 611, -461), Point(-714, 465, -776), Point(-743, 427, -804), Point(-660, -479, -426), Point(832, -632, 460), Point(927, -485, -438), Point(408, 393, -506), Point(466, 436, -512), Point(110, 16, 151), Point(-258, -428, 682), Point(-393, 719, 612), Point(-211, -452, 876), Point(808, -476, -593), Point(-575, 615, 604), Point(-485, 667, 467), Point(-680, 325, -822), Point(-627, -443, -432), Point(872, -547, -609), Point(833, 512, 582), Point(807, 604, 487), Point(839, -516, 451), Point(891, -625, 532), Point(-652, -548, -490), Point(30, -46, -14)],
		];
		let result = part1(&scanners);
		assert_eq!(result, 79);
	}

	#[test]
	#[rustfmt::skip]
	fn part2_example() {
		let scanners = vec![
			vec![Point(404, -588, -901), Point(528, -643, 409), Point(-838, 591, 734), Point(390, -675, -793), Point(-537, -823, -458), Point(-485, -357, 347), Point(-345, -311, 381), Point(-661, -816, -575), Point(-876, 649, 763), Point(-618, -824, -621), Point(553, 345, -567), Point(474, 580, 667), Point(-447, -329, 318), Point(-584, 868, -557), Point(544, -627, -890), Point(564, 392, -477), Point(455, 729, 728), Point(-892, 524, 684), Point(-689, 845, -530), Point(423, -701, 434), Point(7, -33, -71), Point(630, 319, -379), Point(443, 580, 662), Point(-789, 900, -551), Point(459, -707, 401)],
			vec![Point(686, 422, 578), Point(605, 423, 415), Point(515, 917, -361), Point(-336, 658, 858), Point(95, 138, 22), Point(-476, 619, 847), Point(-340, -569, -846), Point(567, -361, 727), Point(-460, 603, -452), Point(669, -402, 600), Point(729, 430, 532), Point(-500, -761, 534), Point(-322, 571, 750), Point(-466, -666, -811), Point(-429, -592, 574), Point(-355, 545, -477), Point(703, -491, -529), Point(-328, -685, 520), Point(413, 935, -424), Point(-391, 539, -444), Point(586, -435, 557), Point(-364, -763, -893), Point(807, -499, -711), Point(755, -354, -619), Point(553, 889, -390)],
			vec![Point(649, 640, 665), Point(682, -795, 504), Point(-784, 533, -524), Point(-644, 584, -595), Point(-588, -843, 648), Point(-30, 6, 44), Point(-674, 560, 763), Point(500, 723, -460), Point(609, 671, -379), Point(-555, -800, 653), Point(-675, -892, -343), Point(697, -426, -610), Point(578, 704, 681), Point(493, 664, -388), Point(-671, -858, 530), Point(-667, 343, 800), Point(571, -461, -707), Point(-138, -166, 112), Point(-889, 563, -600), Point(646, -828, 498), Point(640, 759, 510), Point(-630, 509, 768), Point(-681, -892, -333), Point(673, -379, -804), Point(-742, -814, -386), Point(577, -820, 562)],
			vec![Point(-589, 542, 597), Point(605, -692, 669), Point(-500, 565, -823), Point(-660, 373, 557), Point(-458, -679, -417), Point(-488, 449, 543), Point(-626, 468, -788), Point(338, -750, -386), Point(528, -832, -391), Point(562, -778, 733), Point(-938, -730, 414), Point(543, 643, -506), Point(-524, 371, -870), Point(407, 773, 750), Point(-104, 29, 83), Point(378, -903, -323), Point(-778, -728, 485), Point(426, 699, 580), Point(-438, -605, -362), Point(-469, -447, -387), Point(509, 732, 623), Point(647, 635, -688), Point(-868, -804, 481), Point(614, -800, 639), Point(595, 780, -596)],
			vec![Point(727, 592, 562), Point(-293, -554, 779), Point(441, 611, -461), Point(-714, 465, -776), Point(-743, 427, -804), Point(-660, -479, -426), Point(832, -632, 460), Point(927, -485, -438), Point(408, 393, -506), Point(466, 436, -512), Point(110, 16, 151), Point(-258, -428, 682), Point(-393, 719, 612), Point(-211, -452, 876), Point(808, -476, -593), Point(-575, 615, 604), Point(-485, 667, 467), Point(-680, 325, -822), Point(-627, -443, -432), Point(872, -547, -609), Point(833, 512, 582), Point(807, 604, 487), Point(839, -516, 451), Point(891, -625, 532), Point(-652, -548, -490), Point(30, -46, -14)],
		];
		let result = part2(&scanners);
		assert_eq!(result, 3621);
	}

	#[test]
	#[rustfmt::skip]
	fn part1_overlap() {
		let scanner1 = vec![Point(404, -588, -901), Point(528, -643, 409), Point(-838, 591, 734), Point(390, -675, -793), Point(-537, -823, -458), Point(-485, -357, 347), Point(-345, -311, 381), Point(-661, -816, -575), Point(-876, 649, 763), Point(-618, -824, -621), Point(553, 345, -567), Point(474, 580, 667), Point(-447, -329, 318), Point(-584, 868, -557), Point(544, -627, -890), Point(564, 392, -477), Point(455, 729, 728), Point(-892, 524, 684), Point(-689, 845, -530), Point(423, -701, 434), Point(7, -33, -71), Point(630, 319, -379), Point(443, 580, 662), Point(-789, 900, -551), Point(459, -707, 401)];
		let scanner2 = vec![Point(686, 422, 578), Point(605, 423, 415), Point(515, 917, -361), Point(-336, 658, 858), Point(95, 138, 22), Point(-476, 619, 847), Point(-340, -569, -846), Point(567, -361, 727), Point(-460, 603, -452), Point(669, -402, 600), Point(729, 430, 532), Point(-500, -761, 534), Point(-322, 571, 750), Point(-466, -666, -811), Point(-429, -592, 574), Point(-355, 545, -477), Point(703, -491, -529), Point(-328, -685, 520), Point(413, 935, -424), Point(-391, 539, -444), Point(586, -435, 557), Point(-364, -763, -893), Point(807, -499, -711), Point(755, -354, -619), Point(553, 889, -390)];
		assert!(do_scanners_possibly_overlap(&scanner1, &scanner2));
	}

	#[test]
	#[rustfmt::skip]
	fn part1_overlapping_scanners() {
		let scanners = vec![
			vec![Point(404, -588, -901), Point(528, -643, 409), Point(-838, 591, 734), Point(390, -675, -793), Point(-537, -823, -458), Point(-485, -357, 347), Point(-345, -311, 381), Point(-661, -816, -575), Point(-876, 649, 763), Point(-618, -824, -621), Point(553, 345, -567), Point(474, 580, 667), Point(-447, -329, 318), Point(-584, 868, -557), Point(544, -627, -890), Point(564, 392, -477), Point(455, 729, 728), Point(-892, 524, 684), Point(-689, 845, -530), Point(423, -701, 434), Point(7, -33, -71), Point(630, 319, -379), Point(443, 580, 662), Point(-789, 900, -551), Point(459, -707, 401)],
			vec![Point(686, 422, 578), Point(605, 423, 415), Point(515, 917, -361), Point(-336, 658, 858), Point(95, 138, 22), Point(-476, 619, 847), Point(-340, -569, -846), Point(567, -361, 727), Point(-460, 603, -452), Point(669, -402, 600), Point(729, 430, 532), Point(-500, -761, 534), Point(-322, 571, 750), Point(-466, -666, -811), Point(-429, -592, 574), Point(-355, 545, -477), Point(703, -491, -529), Point(-328, -685, 520), Point(413, 935, -424), Point(-391, 539, -444), Point(586, -435, 557), Point(-364, -763, -893), Point(807, -499, -711), Point(755, -354, -619), Point(553, 889, -390)],
			vec![Point(649, 640, 665), Point(682, -795, 504), Point(-784, 533, -524), Point(-644, 584, -595), Point(-588, -843, 648), Point(-30, 6, 44), Point(-674, 560, 763), Point(500, 723, -460), Point(609, 671, -379), Point(-555, -800, 653), Point(-675, -892, -343), Point(697, -426, -610), Point(578, 704, 681), Point(493, 664, -388), Point(-671, -858, 530), Point(-667, 343, 800), Point(571, -461, -707), Point(-138, -166, 112), Point(-889, 563, -600), Point(646, -828, 498), Point(640, 759, 510), Point(-630, 509, 768), Point(-681, -892, -333), Point(673, -379, -804), Point(-742, -814, -386), Point(577, -820, 562)],
			vec![Point(-589, 542, 597), Point(605, -692, 669), Point(-500, 565, -823), Point(-660, 373, 557), Point(-458, -679, -417), Point(-488, 449, 543), Point(-626, 468, -788), Point(338, -750, -386), Point(528, -832, -391), Point(562, -778, 733), Point(-938, -730, 414), Point(543, 643, -506), Point(-524, 371, -870), Point(407, 773, 750), Point(-104, 29, 83), Point(378, -903, -323), Point(-778, -728, 485), Point(426, 699, 580), Point(-438, -605, -362), Point(-469, -447, -387), Point(509, 732, 623), Point(647, 635, -688), Point(-868, -804, 481), Point(614, -800, 639), Point(595, 780, -596)],
			vec![Point(727, 592, 562), Point(-293, -554, 779), Point(441, 611, -461), Point(-714, 465, -776), Point(-743, 427, -804), Point(-660, -479, -426), Point(832, -632, 460), Point(927, -485, -438), Point(408, 393, -506), Point(466, 436, -512), Point(110, 16, 151), Point(-258, -428, 682), Point(-393, 719, 612), Point(-211, -452, 876), Point(808, -476, -593), Point(-575, 615, 604), Point(-485, 667, 467), Point(-680, 325, -822), Point(-627, -443, -432), Point(872, -547, -609), Point(833, 512, 582), Point(807, 604, 487), Point(839, -516, 451), Point(891, -625, 532), Point(-652, -548, -490), Point(30, -46, -14)],
		];
		let overlapping = find_possibly_overlapping_scanners(&scanners);
		assert_eq!(overlapping, [(0, 1), (1, 3), (1, 4), (2, 4)]);
	}

	#[test]
	#[rustfmt::skip]
	fn part1_find_transformation() {
		let scanner1 = vec![Point(404, -588, -901), Point(528, -643, 409), Point(-838, 591, 734), Point(390, -675, -793), Point(-537, -823, -458), Point(-485, -357, 347), Point(-345, -311, 381), Point(-661, -816, -575), Point(-876, 649, 763), Point(-618, -824, -621), Point(553, 345, -567), Point(474, 580, 667), Point(-447, -329, 318), Point(-584, 868, -557), Point(544, -627, -890), Point(564, 392, -477), Point(455, 729, 728), Point(-892, 524, 684), Point(-689, 845, -530), Point(423, -701, 434), Point(7, -33, -71), Point(630, 319, -379), Point(443, 580, 662), Point(-789, 900, -551), Point(459, -707, 401)];
		let scanner2 = vec![Point(686, 422, 578), Point(605, 423, 415), Point(515, 917, -361), Point(-336, 658, 858), Point(95, 138, 22), Point(-476, 619, 847), Point(-340, -569, -846), Point(567, -361, 727), Point(-460, 603, -452), Point(669, -402, 600), Point(729, 430, 532), Point(-500, -761, 534), Point(-322, 571, 750), Point(-466, -666, -811), Point(-429, -592, 574), Point(-355, 545, -477), Point(703, -491, -529), Point(-328, -685, 520), Point(413, 935, -424), Point(-391, 539, -444), Point(586, -435, 557), Point(-364, -763, -893), Point(807, -499, -711), Point(755, -354, -619), Point(553, 889, -390)];
		let (translation, rotation) = find_transformation(&scanner1, &scanner2).unwrap();
		assert_eq!(translation, Point(68, -1246, -43));
		assert_eq!(rotation, 6);
	}
}
