impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut dp = vec![i32::MAX; cols + 1];
        dp[cols - 1] = 0;

        for r in (0..rows).rev() {
            for c in (0..cols).rev() {
                dp[c] = grid[r][c] + dp[c].min(dp[c + 1]);
            }
        }

        dp[0]
    }
}
