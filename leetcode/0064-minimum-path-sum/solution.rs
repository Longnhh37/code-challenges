impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut prev = vec![0; cols];
        prev[0] = grid[0][0];
        for c in 1..cols {
            prev[c] = prev[c - 1] + grid[0][c];
        }

        for i in 1..rows {
            let mut cur = vec![0; cols];
            cur[0] = prev[0] + grid[i][0];
            for j in 1..cols {
                cur[j] = cur[j - 1].min(prev[j]) + grid[i][j];
            }
            prev = cur;
        }

        prev[cols - 1]
    }
}
