// Solution for this problem by somone else: https://github.com/Jellycious/aoc-2021/blob/main/src/days/day19.rs

use std::{collections::{HashSet, HashMap}, ops::{Add, Sub}};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point(i32, i32, i32);

#[derive(Debug, PartialEq)]
pub struct RotationMatrix(i32, i32, i32, i32, i32, i32, i32, i32, i32);

impl Point {
    fn apply_rotation(&self, t: &RotationMatrix) -> Point {
        return Point(
            self.0 * t.0 + self.1 * t.1 + self.2 * t.2,
            self.0 * t.3 + self.1 * t.4 + self.2 * t.5,
            self.0 * t.6 + self.1 * t.7 + self.2 * t.8
        );
    }

    fn sqr_dist(&self, other: &Point) -> i32 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        let dz = self.2 - other.2;
        return dx*dx + dy*dy + dz*dz;
    }
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

fn parse_point(line: &str) -> Point {
    let nums: Vec<&str> = line.splitn(3, ',').collect();
    let x = nums[0].parse().unwrap();
    let y = nums[1].parse().unwrap();
    let z = nums[2].parse().unwrap();
    return Point(x, y, z);
}

fn parse_scanner(input: &str) -> Vec<Point> {
    let mut beacons = Vec::new();
    for line in input.lines().skip(1) {
        beacons.push(parse_point(line));
    }
    return beacons;
}

pub fn parse_input(input: &str) -> Vec<Vec<Point>> {
    let mut scanners = Vec::new();
    for scanner_section in input.split("\n\n") {
        scanners.push(parse_scanner(scanner_section));
    }
    return scanners;
}

fn generate_rotations() -> Vec<RotationMatrix> {
    let mut transforms = Vec::new();

    // +z axis
    transforms.push(RotationMatrix( 1, 0, 0,   0,  1, 0,    0, 0, 1));
    transforms.push(RotationMatrix( 0, 1, 0,  -1,  0, 0,    0, 0, 1));
    transforms.push(RotationMatrix(-1, 0, 0,   0, -1, 0,    0, 0, 1));
    transforms.push(RotationMatrix(0, -1, 0,   1,  0, 0,    0, 0, 1));

    // -z axis
    transforms.push(RotationMatrix( 1, 0, 0,   0, -1, 0,    0, 0, -1));
    transforms.push(RotationMatrix( 0, 1, 0,   1,  0, 0,    0, 0, -1));
    transforms.push(RotationMatrix(-1, 0, 0,   0,  1, 0,    0, 0, -1));
    transforms.push(RotationMatrix(0, -1, 0,  -1,  0, 0,    0, 0, -1));

    // +y axis
    transforms.push(RotationMatrix( 1, 0, 0,   0, 0, -1,    0, 1, 0));
    transforms.push(RotationMatrix( 0, 0, 1,   1, 0,  0,    0, 1, 0));
    transforms.push(RotationMatrix(-1, 0, 0,   0, 0,  1,    0, 1, 0));
    transforms.push(RotationMatrix( 0, 0,-1,  -1, 0,  0,    0, 1, 0));

    // -y axis
    transforms.push(RotationMatrix( 1, 0, 0,   0, 0,  1,    0,-1, 0));
    transforms.push(RotationMatrix( 0, 0, 1,  -1, 0,  0,    0,-1, 0));
    transforms.push(RotationMatrix(-1, 0, 0,   0, 0, -1,    0,-1, 0));
    transforms.push(RotationMatrix( 0, 0,-1,   1, 0,  0,    0,-1, 0));
    
    // +x axis
    transforms.push(RotationMatrix( 0,-1, 0,   0, 0, -1,    1, 0, 0));
    transforms.push(RotationMatrix( 0, 0,-1,   0, 1,  0,    1, 0, 0));
    transforms.push(RotationMatrix( 0, 1, 0,   0, 0,  1,    1, 0, 0));
    transforms.push(RotationMatrix( 0, 0, 1,   0,-1,  0,    1, 0, 0));

    // -x axis
    transforms.push(RotationMatrix( 0, 1, 0,   0, 0, -1,   -1, 0, 0));
    transforms.push(RotationMatrix( 0, 0, 1,   0, 1,  0,   -1, 0, 0));
    transforms.push(RotationMatrix( 0,-1, 0,   0, 0,  1,   -1, 0, 0));
    transforms.push(RotationMatrix( 0, 0,-1,   0,-1,  0,   -1, 0, 0));

    return transforms;
}

fn find_distances_pairs<'a>(points: &'a Vec<Point>) -> Vec<(i32, &'a Point, &'a Point)> {
    let mut distances = Vec::new();
    let n = points.len();
    for i in 0..n-1 {
        for j in i+1..n {
            let p1 = &points[i];
            let p2 = &points[j];
            let dist = p1.sqr_dist(&p2);
            distances.push((dist, p1, p2));
        }
    }
    return distances;
}

fn find_common_points<'a, 'b>(scanner1: &'a Vec<Point>, scanner2: &'b Vec<Point>) -> (Vec<Point>, Vec<Point>) {
    let mut common_points1: Vec<Point> = Vec::new();
    let mut common_points2: Vec<Point> = Vec::new();

    for pair1 in find_distances_pairs(scanner1) {
        for pair2 in find_distances_pairs(scanner2) {
            if pair1.0 == pair2.0 {
                if !common_points1.contains(pair1.1) {
                    common_points1.push(*pair1.1);
                }
                if !common_points1.contains(pair1.2) {
                    common_points1.push(*pair1.2);
                }
                if !common_points2.contains(pair2.1) {
                    common_points2.push(*pair2.1);
                }
                if !common_points2.contains(pair2.2) {
                    common_points2.push(*pair2.2);
                }
            }
        }
    }

    return (common_points1, common_points2);
}

fn apply_offset_rotation(points: &Vec<Point>, offset: &Point, rotation: &RotationMatrix) -> Vec<Point> {
    points.iter()
        .map(|p| p.apply_rotation(&rotation) + *offset)
        .collect()
}

fn find_offset_rotation(mut points1: Vec<Point>, points2: Vec<Point>) -> Option<(Point, RotationMatrix)> {
    points1.sort();

    'outer: for rotation in generate_rotations() {
        let mut transformed_points: Vec<_> = points2.iter()
            .map(|p| p.apply_rotation(&rotation))
            .collect();
        transformed_points.sort();

        let offset = points1[0] - transformed_points[0];
        if rotation == RotationMatrix(-1, 0, 0, 0, -1, 0, 0, 0, 1) {
            println!("\n\nTransform: {:?}", rotation);
            println!("Offset: {:?}\n", offset);
            println!("{:?}\n", points1);
            // println!("{:?}", transformed_points);
        }

        for i in 1..points1.len() {
            let diff = points1[i] - transformed_points[i];
            if offset != diff {
                continue 'outer;
            }
        }

        return Some((offset, rotation));
    }

    return None;
}

fn align_scanners(scanners: &mut Vec<Vec<Point>>) {
    let mut unaligned = HashSet::new();
    let mut unchecked = vec![0];

    for i in 1..scanners.len() {
        unaligned.insert(i);
    }

    while !unchecked.is_empty() {

        let aligned_id = unchecked.pop().unwrap();
        for i in unaligned.clone() {
            let unaligned_scanner = &scanners[i];
            let (common_points1, common_points2) = find_common_points(&scanners[aligned_id], unaligned_scanner);
            if common_points1.len() >= 12 {
                let transform = find_offset_rotation(common_points1, common_points2);
                if let Some((offset, rotation)) = transform {
                    scanners[i] = apply_offset_rotation(&scanners[i], &offset, &rotation);
                    unchecked.push(i);
                    unaligned.remove(&i);
                }
            }
        }
    }
    
}

fn find_unique_points(scanners: &Vec<Vec<Point>>) -> Vec<Point> {
    let mut cloned = scanners.clone();

    align_scanners(&mut cloned);

    let mut point_set = HashSet::new();
    for scanner in cloned {
        for point in scanner {
            point_set.insert(point);
        }
    }

    return point_set.into_iter().collect();
}

pub fn part1(scanners: &Vec<Vec<Point>>) -> u32 {
    find_unique_points(scanners).len() as u32
}

pub fn part2(scanners: &Vec<Vec<Point>>) -> u32 {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let scanners = vec![
            vec![Point(404,-588,-901), Point(528,-643,409), Point(-838,591,734), Point(390,-675,-793), Point(-537,-823,-458), Point(-485,-357,347), Point(-345,-311,381), Point(-661,-816,-575), Point(-876,649,763), Point(-618,-824,-621), Point(553,345,-567), Point(474,580,667), Point(-447,-329,318), Point(-584,868,-557), Point(544,-627,-890), Point(564,392,-477), Point(455,729,728), Point(-892,524,684), Point(-689,845,-530), Point(423,-701,434), Point(7,-33,-71), Point(630,319,-379), Point(443,580,662), Point(-789,900,-551), Point(459,-707,401)],
            vec![Point(686,422,578), Point(605,423,415), Point(515,917,-361), Point(-336,658,858), Point(95,138,22), Point(-476,619,847), Point(-340,-569,-846), Point(567,-361,727), Point(-460,603,-452), Point(669,-402,600), Point(729,430,532), Point(-500,-761,534), Point(-322,571,750), Point(-466,-666,-811), Point(-429,-592,574), Point(-355,545,-477), Point(703,-491,-529), Point(-328,-685,520), Point(413,935,-424), Point(-391,539,-444), Point(586,-435,557), Point(-364,-763,-893), Point(807,-499,-711), Point(755,-354,-619), Point(553,889,-390)],
            vec![Point(649,640,665), Point(682,-795,504), Point(-784,533,-524), Point(-644,584,-595), Point(-588,-843,648), Point(-30,6,44), Point(-674,560,763), Point(500,723,-460), Point(609,671,-379), Point(-555,-800,653), Point(-675,-892,-343), Point(697,-426,-610), Point(578,704,681), Point(493,664,-388), Point(-671,-858,530), Point(-667,343,800), Point(571,-461,-707), Point(-138,-166,112), Point(-889,563,-600), Point(646,-828,498), Point(640,759,510), Point(-630,509,768), Point(-681,-892,-333), Point(673,-379,-804), Point(-742,-814,-386), Point(577,-820,562)],
            vec![Point(-589,542,597), Point(605,-692,669), Point(-500,565,-823), Point(-660,373,557), Point(-458,-679,-417), Point(-488,449,543), Point(-626,468,-788), Point(338,-750,-386), Point(528,-832,-391), Point(562,-778,733), Point(-938,-730,414), Point(543,643,-506), Point(-524,371,-870), Point(407,773,750), Point(-104,29,83), Point(378,-903,-323), Point(-778,-728,485), Point(426,699,580), Point(-438,-605,-362), Point(-469,-447,-387), Point(509,732,623), Point(647,635,-688), Point(-868,-804,481), Point(614,-800,639), Point(595,780,-596)],
            vec![Point(727,592,562), Point(-293,-554,779), Point(441,611,-461), Point(-714,465,-776), Point(-743,427,-804), Point(-660,-479,-426), Point(832,-632,460), Point(927,-485,-438), Point(408,393,-506), Point(466,436,-512), Point(110,16,151), Point(-258,-428,682), Point(-393,719,612), Point(-211,-452,876), Point(808,-476,-593), Point(-575,615,604), Point(-485,667,467), Point(-680,325,-822), Point(-627,-443,-432), Point(872,-547,-609), Point(833,512,582), Point(807,604,487), Point(839,-516,451), Point(891,-625,532), Point(-652,-548,-490), Point(30,-46,-14)]
        ];
        let result = part1(&scanners);
        assert_eq!(result, 79);
    }
}

