impl Solution {
    pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut prev = grid[0].clone();
        let mut cur = vec![0; cols];

        for r in 1..rows {
            for c in 0..cols {
                let mut best = i32::MAX;
                for i in 0..cols {
                    let cost = prev[i] + move_cost[grid[r - 1][i] as usize][c];
                    best = best.min(cost);
                }
                cur[c] = best + grid[r][c];
            }
            std::mem::swap(&mut prev, &mut cur);
        }

        prev.into_iter().min().unwrap()
    }
}
