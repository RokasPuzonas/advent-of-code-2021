
pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn find_low_points(grid: Vec<Vec<u32>>) -> Vec<u32> {
    let mut low_points = Vec::new();
    let height = grid.len();
    for i in 0..height {
        let width = grid[i].len();
        for j in 0..width {
            if (i == 0        || grid[i-1][j] > grid[i][j])
            && (i == height-1 || grid[i+1][j] > grid[i][j])
            && (j == 0        || grid[i][j-1] > grid[i][j])
            && (j == width-1  || grid[i][j+1] > grid[i][j]) {
                low_points.push(grid[i][j]);
            }
        }
    }
    return low_points;
}

pub fn part1(grid: Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    for low_point in find_low_points(grid) {
        sum += low_point + 1;
    }
    return sum;
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
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8]
        ];
        let result = part1(input);
        assert_eq!(result, 15);
    }
}
