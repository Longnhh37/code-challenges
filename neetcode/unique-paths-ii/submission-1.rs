impl Solution {
    pub fn unique_paths_with_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();
        let mut dp = vec![0_i32; n];
        dp[0] = 1;

        for row in grid.iter() {
            for j in 0..n {
                if row[j] == 1 {
                    dp[j] = 0;
                } else if j > 0 {
                    dp[j] += dp[j - 1];
                }
            }
        }

        dp[n - 1]
    }
}
