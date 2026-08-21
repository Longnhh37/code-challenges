impl Solution {
    pub fn unique_paths_with_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![0_i32; n]; m];

        for j in 0..n {
            if grid[0][j] == 1 {
                break;
            }
            dp[0][j] = 1;
        }

        for i in 0..m {
            if grid[i][0] == 1 {
                break;
            }
            dp[i][0] = 1;
        }

        for i in 1..m {
            for j in 1..n {
                if grid[i][j] == 1 {
                    continue;
                }
                if grid[i][j - 1] != 1 {
                    dp[i][j] += dp[i][j - 1];
                }
                if grid[i - 1][j] != 1 {
                    dp[i][j] += dp[i - 1][j];
                }
            }
        }

        dp[m - 1][n - 1]
    }
}
