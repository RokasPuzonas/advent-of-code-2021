use std::collections::{HashMap, HashSet};
use priority_queue::PriorityQueue;


#[derive(Debug)]
pub struct Grid {
    rows: u32,
    cols: u32,
    data: Vec<u32>
}

pub fn parse_input(input: &str) -> Grid {
    let mut data = Vec::new();
    let mut rows = 0;
    let mut cols = 0;
    for line in input.lines() {
        rows += 1;
        for c in line.chars() {
            if rows == 1 { cols += 1 }
            data.push(c.to_digit(10).unwrap());
        }
    }
    return Grid { rows, cols, data };
}

fn find_shortest_path_cost(grid: &Grid) -> u32 {
    let mut total_costs: HashMap<(u32, u32), u32> = HashMap::new();
    let mut min_pq: PriorityQueue<(u32, u32), i32> = PriorityQueue::new();
    let mut visited: HashSet<(u32, u32)> = HashSet::new();
    let neighbour_offsets = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0)
    ];
    min_pq.push((0, 0), 0);
    total_costs.insert((0, 0), 0);

    while !min_pq.is_empty() {
        let new_smallest = min_pq.pop().unwrap().0;
        visited.insert(new_smallest);

        for offset in neighbour_offsets {
            let neighbour_row = new_smallest.0 as i32 + offset.0;
            let neighbour_col = new_smallest.1 as i32 + offset.1;
            if !(neighbour_row >= 0 && neighbour_col >= 0 && neighbour_row < grid.rows as i32 && neighbour_col < grid.cols as i32) {
                continue;
            }
            let neighbour = (neighbour_row as u32, neighbour_col as u32);

            if visited.contains(&neighbour) {
                continue;
            }

            let alt_distance = grid.data[(neighbour_row as u32 * grid.cols + neighbour_col as u32) as usize];
            let alt_path = total_costs.get(&new_smallest).unwrap_or(&u32::MAX) + alt_distance;
            if alt_path < *total_costs.get(&neighbour).unwrap_or(&u32::MAX) {
                total_costs.insert(neighbour, alt_path);
                min_pq.push_decrease(neighbour, -(alt_path as i32));
            }
        }
    }
    
    return *total_costs.get(&(grid.rows-1, grid.cols-1)).unwrap();
}

pub fn part1(grid: &Grid) -> u32 {
    find_shortest_path_cost(grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let grid = parse_input("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581");
        let result = part1(&grid);
        assert_eq!(result, 40);
    }
}
