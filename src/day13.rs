use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dot(u32, u32);

#[derive(Debug)]
pub enum Fold {
    X(u32),
    Y(u32),
}

pub struct InputData {
    dots: Vec<Dot>,
    folds: Vec<Fold>
}

fn parse_dot(line: &str) -> Dot {
    let (x, y) = line.split_once(',').unwrap();
    return Dot(
        x.parse().unwrap(),
        y.parse().unwrap()
    );
}

fn parse_fold(line: &str) -> Fold {
    let (axis, coordinate_str) = line.split_once('=').unwrap();
    let coordinate = coordinate_str.parse().unwrap();
    match axis {
        "fold along x" => Fold::X(coordinate),
        "fold along y" => Fold::Y(coordinate),
        _ => unreachable!("Unable to parse fold direction")
    }
}

pub fn parse_input(input: &str) -> InputData {
    let (dots_section, folds_section) = input.split_once("\n\n").unwrap();

    let dots = dots_section.lines()
        .map(parse_dot)
        .collect();
    let folds = folds_section.lines()
        .map(parse_fold)
        .collect();

    return InputData {
        dots,
        folds
    };
}

fn perform_fold(dots: &HashSet<Dot>, fold: &Fold) -> HashSet<Dot> {
    let mut folded_dots = HashSet::new();

    for dot in dots.iter() {
        let folded_dot = match fold {
            Fold::X(x) => {
                if dot.0 > *x {
                    Dot(2*x - dot.0, dot.1)
                } else {
                    dot.clone()
                }
            },
            Fold::Y(y) => {
                if dot.1 > *y {
                    Dot(dot.0, 2*y - dot.1)
                } else {
                    dot.clone()
                }
            },
        };
        folded_dots.insert(folded_dot);
    }

    return folded_dots;
}

pub fn part1(input: &InputData) -> usize {
    let mut folded_dots = HashSet::new();
    for dot in &input.dots {
        folded_dots.insert(dot.clone());
    }
    folded_dots = perform_fold(&folded_dots, &input.folds[0]);
    folded_dots.len() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let dots = vec![
            Dot(6, 10),
            Dot(0, 14),
            Dot(9, 10),
            Dot(0, 3),
            Dot(10, 4),
            Dot(4, 11),
            Dot(6, 0),
            Dot(6, 12),
            Dot(4, 1),
            Dot(0, 13),
            Dot(10, 12),
            Dot(3, 4),
            Dot(3, 0),
            Dot(8, 4),
            Dot(1, 10),
            Dot(2 ,14),
            Dot(8 ,10),
            Dot(9, 0)
        ];
        let folds = vec![
            Fold::Y(7),
            Fold::X(5),
        ];
        let result = part1(&InputData{ dots, folds });
        assert_eq!(result, 17);
    }
}
