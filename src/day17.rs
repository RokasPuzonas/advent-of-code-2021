
pub struct Rect {
    x0: i32, x1: i32,
    y0: i32, y1: i32,
}

pub fn parse_input(input: &str) -> Rect {
    let (x_part, y_part) = input.strip_suffix("\n")
        .or(Some(input))
        .unwrap()
        .strip_prefix("target area: ")
        .unwrap()
        .split_once(", ")
        .unwrap();
    let (x0, x1) = x_part[2..].split_once("..").unwrap();
    let (y0, y1) = y_part[2..].split_once("..").unwrap();
    return Rect {
        x0: x0.parse().unwrap(),
        x1: x1.parse().unwrap(),
        y0: y0.parse().unwrap(),
        y1: y1.parse().unwrap()
    }
}

fn sign(x: i32) -> i32 {
    if x > 0 { 1 } else if x < 0 { -1 } else { 0 }
}

fn is_overshot(px: i32, py: i32, target: &Rect) -> bool {
    px > target.x1 || py < target.y0
}

fn is_in_rect(px: i32, py: i32, target: &Rect) -> bool {
    target.x0 <= px && px <= target.x1 &&
    target.y0 <= py && py <= target.y1
}

fn simulate(target: &Rect, initial_vx: i32, initial_vy: i32) -> i32 {
    let mut px = 0;
    let mut py = 0;
    let mut vx = initial_vx;
    let mut vy = initial_vy;
    let mut maxy = 0;

    while !is_overshot(px, py, target) {
        px += vx;
        py += vy;

        if is_in_rect(px, py, target) {
            return maxy;
        }
        
        maxy = maxy.max(py);

        vx -= sign(vx);
        vy -= 1;
    }

    return -1;
}

pub fn part1(target: &Rect) -> i32 {
    let mut maxy = 0;
    for vx in 0..target.x1 {
        for vy in target.y0..-target.y0 {
            maxy = maxy.max(simulate(target, vx, vy));
        }
    }
    return maxy;
}

pub fn part2(target: &Rect) -> i32 {
    let mut count = 0;
    for vx in 0..=target.x1 {
        for vy in target.y0..-target.y0 {
            if simulate(target, vx, vy) >= 0 {
                count += 1;
            }
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let target = parse_input("target area: x=20..30, y=-10..-5");
        let result = part1(&target);
        assert_eq!(result, 45);
    }

    #[test]
    fn part2_example() {
        let target = parse_input("target area: x=20..30, y=-10..-5");
        let result = part2(&target);
        assert_eq!(result, 112);
    }
}

