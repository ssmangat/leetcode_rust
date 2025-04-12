use std::collections::HashMap;
pub struct Solution {}

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut grid_map: HashMap<Vec<i32>, i32> = HashMap::new();
        for row in &grid {
            *grid_map.entry(row.clone()).or_insert(0) += 1;
        }
        let mut counter: i32 = 0;
        for j in 0..n {
            let mut column = Vec::with_capacity(n);
            for i in 0..n {
                column.push(grid[i][j]);
            }
            if let Some(c) = grid_map.get(&column) {
                counter += c;
            }
        }
        counter
    }
}
