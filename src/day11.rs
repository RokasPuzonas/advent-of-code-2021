use arrayvec::ArrayVec;

pub fn parse_input(input: &str) -> [[u32; 10]; 10] {
    input.lines()
        .map(|s| s.chars()
             .map(|s| s.to_digit(10).unwrap())
             .collect::<ArrayVec<u32, 10>>()
             .into_inner()
             .unwrap())
        .collect::<ArrayVec<[u32; 10], 10>>()
        .into_inner()
        .unwrap()
}

fn display_grid(grid: &[[u32; 10]; 10]) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            print!("{:X}", grid[i][j]);
        }
        print!("\n");
    }
    print!("\n");
}

fn bump_energy(grid: &mut [[u32; 10]; 10]) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            grid[i][j] += 1;
        }
    }
}

fn bump_energy_around(grid: &mut [[u32; 10]; 10], i: usize, j: usize) {
    if j > 0 {
        grid[i+0][j-1] += 1;
    }
    if j < 9 {
        grid[i+0][j+1] += 1;
    }
    if i > 0 {
        grid[i-1][j+0] += 1;
    }
    if i < 9 {
        grid[i+1][j+0] += 1;
    }

    if i > 0 && j > 0 {
        grid[i-1][j-1] += 1;
    }
    if i < 9 && j > 0 {
        grid[i+1][j-1] += 1;
    }
    if i > 0 && j < 9 {
        grid[i-1][j+1] += 1;
    }
    if i < 9 && j < 9 {
        grid[i+1][j+1] += 1;
    }
}

fn perform_flashes(grid: &mut [[u32; 10]; 10]) -> u32 {
    let mut flashes = 0;
    let mut has_flashed: [[bool; 10]; 10] = [[false; 10]; 10];
    let mut anyone_flashed = true;

    while anyone_flashed {
        anyone_flashed = false;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] > 9 && !has_flashed[i][j] {
                    flashes += 1;
                    has_flashed[i][j] = true;
                    anyone_flashed = true;
                    bump_energy_around(grid, i, j);
                }
            }
        }
    }

    return flashes;
}

fn reset_energy(grid: &mut [[u32; 10]; 10]) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] > 9 {
                grid[i][j] = 0;
            }
        }
    }
}

fn do_step(grid: &mut [[u32; 10]; 10]) -> u32 {
    bump_energy(grid);
    let flashes = perform_flashes(grid);
    reset_energy(grid);
    return flashes;
}

fn has_all_zeros(grid: &[[u32; 10]; 10]) -> bool {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] != 0 {
                return false;
            }
        }
    }
    return true;
}

pub fn part1(grid: &[[u32; 10]; 10]) -> u32 {
    let mut flashes = 0;
    let mut active_grid = grid.clone();
    for _ in 0..100 {
        flashes += do_step(&mut active_grid);
    }
    return flashes;
}

pub fn part2(grid: &[[u32; 10]; 10]) -> u32 {
    let mut active_grid = grid.clone();
    let mut step = 0;
    while !has_all_zeros(&active_grid) {
        do_step(&mut active_grid);
        step += 1;
    }
    return step;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = [
            [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            [5, 2, 8, 3, 7, 5, 1, 5, 2, 6]
       ];
        let result = part1(&input);
        assert_eq!(result, 1656);
    }

    #[test]
    fn part2_example() {
        let input = [
            [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            [5, 2, 8, 3, 7, 5, 1, 5, 2, 6]
       ];
        let result = part2(&input);
        assert_eq!(result, 195);
    }
}

