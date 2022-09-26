use std::{ops::{Sub, Mul, Add, Neg}, collections::{HashMap, HashSet}, vec, fmt};

// https://i.imgur.com/Ff1vGT9.png
const ROTATIONS: [Rotation; 24] = [
 	Rotation( 1, 0, 0, 0, 1, 0, 0, 0, 1), // (x ,y ,z)
 	Rotation( 1, 0, 0, 0, 0, 1, 0,-1, 0), // (x ,z ,-y)
 	Rotation( 1, 0, 0, 0,-1, 0, 0, 0,-1), // (x ,-y,-z)
 	Rotation( 1, 0, 0, 0, 0,-1, 0, 1, 0), // (x ,-z,y  )
	Rotation(-1, 0, 0, 0,-1, 0, 0, 0, 1), // (-x,-y,z)
	Rotation(-1, 0, 0, 0, 0, 1, 0, 1, 0), // (-x,z ,y )
	Rotation(-1, 0, 0, 0, 1, 0, 0, 0,-1), // (-x,y ,-z)
	Rotation(-1, 0, 0, 0, 0,-1, 0,-1, 0), // (-x,-z,-y )
 	Rotation( 0, 1, 0, 0, 0, 1, 1, 0, 0), // (y ,z ,x)
 	Rotation( 0, 1, 0, 1, 0, 0, 0, 0,-1), // (y ,x ,-z)
 	Rotation( 0, 1, 0, 0, 0,-1,-1, 0, 0), // (y ,-z,-x)
 	Rotation( 0, 1, 0,-1, 0, 0, 0, 0, 1), // (y ,-x,z  )
 	Rotation( 0,-1, 0, 0, 0,-1, 1, 0, 0), // (-y,-z,x)
 	Rotation( 0,-1, 0, 1, 0, 0, 0, 0, 1), // (-y,x ,z )
 	Rotation( 0,-1, 0, 0, 0, 1,-1, 0, 0), // (-y,z ,-x)
 	Rotation( 0,-1, 0,-1, 0, 0, 0, 0,-1), // (-y,-x,-z )
 	Rotation( 0, 0, 1, 1, 0, 0, 0, 1, 0), // (z ,x ,y)
 	Rotation( 0, 0, 1, 0, 1, 0,-1, 0, 0), // (z ,y ,-x)
 	Rotation( 0, 0, 1,-1, 0, 0, 0,-1, 0), // (z ,-x,-y)
 	Rotation( 0, 0, 1, 0,-1, 0, 1, 0, 0), // (z ,-y,x  )
 	Rotation( 0, 0,-1,-1, 0, 0, 0, 1, 0), // (-z,-x,y)
 	Rotation( 0, 0,-1, 0, 1, 0, 1, 0, 0), // (-z,y ,x )
 	Rotation( 0, 0,-1, 1, 0, 0, 0,-1, 0), // (-z,x ,-y)
 	Rotation( 0, 0,-1, 0,-1, 0,-1, 0, 0), // (-z,-y,-x)
];

#[derive(Clone, Copy, PartialEq, Eq)]
struct Rotation(i32, i32, i32, i32, i32, i32, i32, i32, i32);

impl Rotation {
	fn identity() -> Rotation {
		Rotation(1, 0, 0, 0, 1, 0, 0, 0, 1)
	}

	fn transpose(self) -> Rotation {
		Rotation(
			self.0,
			self.3,
			self.6,

			self.1,
			self.4,
			self.7,

			self.2,
			self.5,
			self.8,
		)
	}
}

impl fmt::Debug for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			fn write_param(f: &mut fmt::Formatter<'_>, x: i32, y: i32, z: i32) -> fmt::Result {
				if x != 0 {
					write!(f, "{}", if x < 0 {"-x"} else {"x"})?;
				} else if y != 0 {
					write!(f, "{}", if y < 0 {"-y"} else {"y"})?;
				} else if z != 0 {
					write!(f, "{}", if z < 0 {"-z"} else {"z"})?;
				} else {
					write!(f, "0")?;
				}
				Ok(())
			}

			write!(f, "Rotation(")?;
			write_param(f, self.0, self.1, self.2)?;
			write!(f, ",")?;
			write_param(f, self.3, self.4, self.5)?;
			write!(f, ",")?;
			write_param(f, self.6, self.7, self.8)?;
			write!(f, ")")?;
			Ok(())
    }
}

impl Mul<Point> for Rotation {
	type Output = Point;

	fn mul(self, rhs: Point) -> Self::Output {
		Point(
			self.0*rhs.0 + self.1*rhs.1 + self.2*rhs.2,
			self.3*rhs.0 + self.4*rhs.1 + self.5*rhs.2,
			self.6*rhs.0 + self.7*rhs.1 + self.8*rhs.2
		)
	}
}

impl Mul for Rotation {
	type Output = Rotation;

	// https://www.fhybea.com/en/multiplication-matrix-3x3.html
	fn mul(self, rhs: Self) -> Self::Output {
		Rotation(
			(self.0 * rhs.0) + (self.1 * rhs.3) + (self.2 * rhs.6),
			(self.0 * rhs.1) + (self.1 * rhs.4) + (self.2 * rhs.7),
			(self.0 * rhs.2) + (self.1 * rhs.5) + (self.2 * rhs.8),
			(self.3 * rhs.0) + (self.4 * rhs.3) + (self.5 * rhs.6),
			(self.3 * rhs.1) + (self.4 * rhs.4) + (self.5 * rhs.7),
			(self.3 * rhs.2) + (self.4 * rhs.5) + (self.5 * rhs.8),
			(self.6 * rhs.0) + (self.7 * rhs.3) + (self.8 * rhs.6),
			(self.6 * rhs.1) + (self.7 * rhs.4) + (self.8 * rhs.7),
			(self.6 * rhs.2) + (self.7 * rhs.5) + (self.8 * rhs.8)
		)
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point(i32, i32, i32);

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
				Point(-self.0, -self.1, -self.2)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				write!(f, "Point({},{},{})", self.0, self.1, self.2)?;
				Ok(())
    }
}

impl Point {
	fn mag_sqrd(&self) -> i32 {
		self.0*self.0 + self.1*self.1 + self.2*self.2
	}

	fn manhattan(&self, other: &Point) -> i32 {
		(self.0-other.0).abs() + (self.1-other.1).abs() + (self.2-other.2).abs()
	}
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
				Point (
					self.0 - rhs.0,
					self.1 - rhs.1,
					self.2 - rhs.2
				)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
				Point (
					self.0 + rhs.0,
					self.1 + rhs.1,
					self.2 + rhs.2
				)
    }
}

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
	let mut scanners = Vec::new(); for scanner_section in input.split("\n\n") {
		scanners.push(parse_scanner(scanner_section));
	}
	scanners
}

fn calc_beacon_pairs(beacons: &[Point]) -> HashMap<(usize, usize), i32> {
	let mut diffs = HashMap::new();
	for (i, p1) in beacons.iter().enumerate() {
		for (j, p2) in beacons.iter().enumerate().skip(i+1) {
			let diff = (*p1 - *p2).mag_sqrd();
			diffs.insert((i, j), diff);
		}
	}
	diffs
}

fn find_correct_transform(
		points1: &[Point], diffs1: &HashMap<(usize, usize), i32>,
		points2: &[Point], diffs2: &HashMap<(usize, usize), i32>
	) -> Option<(Rotation, Point)> {

	for rot in ROTATIONS {
		for (pair1, diff1) in diffs1 {
			for (pair2, diff2) in diffs2 {
				if diff1 == diff2 {
					let base1 = points1[pair1.0];
					let base2 = points2[pair2.0];
					let normalized1 = points1.iter().map(|p| *p - base1).collect::<HashSet<_>>();
					let normalized2 = points2.iter().map(|p| rot.mul(*p - base2)).collect::<HashSet<_>>();

					if normalized1.intersection(&normalized2).count() >= 12 {
						let offset = base1 - rot.mul(base2);
						// dbg!(normalized1.intersection(&normalized2).count());
						// let a = points1.iter().copied().collect::<HashSet<_>>();
						// let b = points2.iter().map(|p| rot.mul(*p) + offset).collect::<HashSet<_>>();
						// dbg!(a.intersection(&b).count());
						return Some((rot, offset))
					}
				}
			}
		}
	}

	None
}

fn calc_transforms(scanners: &[Vec<Point>]) -> Vec<(Rotation, Point)> {
	let mut diffs = vec![];
	for scanner in scanners {
		diffs.push(calc_beacon_pairs(scanner))
	}

	let mut relative_transforms = HashMap::new();
	{
		let dist_diffs = diffs.iter()
			.map(|m| m.values().collect::<HashSet<_>>())
			.collect::<Vec<_>>();

		for (i, pairs1) in dist_diffs.iter().enumerate() {
			for (j, pairs2) in dist_diffs.iter().enumerate().skip(i+1) {
				if pairs1.intersection(pairs2).count() >= 66 {
					let trans = find_correct_transform(
						&scanners[i], &diffs[i],
						&scanners[j], &diffs[j]
					);
					if let Some((rot, offset)) = trans {
						relative_transforms.entry(i)
							.or_insert(vec![])
							.push((j, rot, offset));

						let inv_rot = rot.transpose();

						relative_transforms.entry(j)
							.or_insert(vec![])
							.push((i, inv_rot, inv_rot.mul(-offset)));
					}
				}
			}
		}
	}

	let mut transforms = vec![];
	for _ in 0..scanners.len() {
		transforms.push((Rotation::identity(), Point(0, 0, 0)));
	}

	let mut stack = vec![(0, Rotation::identity(), Point(0, 0, 0))];
	let mut visited = HashSet::new();
	while !stack.is_empty() {
		let (id, rot, offset) = stack.pop().unwrap();

		if visited.contains(&id) { continue; }
		visited.insert(id);

		for nbr in relative_transforms.get(&id).unwrap() {
			if visited.contains(&nbr.0) { continue; }

			let new_offset = rot.mul(nbr.2) + offset;
			let new_rot = rot.mul(nbr.1);
			stack.push((nbr.0, new_rot, new_offset));
			transforms[nbr.0] = (new_rot, new_offset);
		}
	}

	transforms
}

pub fn part1(scanners: &[Vec<Point>]) -> u32 {
	let transforms = calc_transforms(scanners);

	let mut all_beacons = HashSet::new();
	for (i, (rot, offset)) in transforms.iter().enumerate() {
		for scanner in &scanners[i] {
			all_beacons.insert(rot.mul(*scanner) + *offset);
		}
	}

	all_beacons.len() as u32
}

pub fn part2(scanners: &[Vec<Point>]) -> i32 {
	let transforms = calc_transforms(scanners);

	let mut result = 0;
	for (i, (_, offset1)) in transforms.iter().enumerate() {
		for (_, offset2) in transforms.iter().skip(i+1) {
			result = result.max(offset1.manhattan(offset2))
		}
	}

	result
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
	fn rotation_matrices() {
		let p = Point(1, 2, 3);
		let Point(x, y, z) = p;
		let ref_rotations = vec![
			Point(x ,y ,z), Point(x ,z,-y), Point(x ,-y,-z), Point(x ,-z,y  ),
			Point(-x,-y,z), Point(-x,z,y ), Point(-x,y ,-z), Point(-x,-z,-y ),
			Point(y ,z ,x), Point(y ,x,-z), Point(y ,-z,-x), Point(y ,-x,z  ),
			Point(-y,-z,x), Point(-y,x,z ), Point(-y,z ,-x), Point(-y,-x,-z ),
			Point(z ,x ,y), Point(z ,y,-x), Point(z ,-x,-y), Point(z ,-y,x  ),
			Point(-z,-x,y), Point(-z,y,x ), Point(-z,x ,-y), Point(-z,-y,-x)
		];
		let calculated: Vec<_> = ROTATIONS.iter().map(|rot| rot.mul(p)).collect();
		for (i, rot) in calculated.iter().enumerate() {
			assert_eq!(*rot, ref_rotations[i]);
		}
	}

	#[test]
	#[rustfmt::skip]
	fn multiply_matrices() {
		let a = Rotation(2, 3, 1, 7, 4, 1, 9, -2, 1);
		let b = Rotation(9, -2, -1, 5, 7, 3, 8, 1, 0);
		let c = a.mul(b);
		assert_eq!(c, Rotation(41, 18, 7,  91, 15, 5, 79, -31, -15));
	}
}
