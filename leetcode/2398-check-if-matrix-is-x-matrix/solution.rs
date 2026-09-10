impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();

        for i in 0..n {
            for j in 0..n {
                if i - j == 0 {
                    if grid[i][j] == 0 {
                        return false;
                    } else {
                        continue;
                    }
                }
                if i + j == n - 1 {
                    if grid[i][j] == 0 {
                        return false;
                    } else {
                        continue;
                    }
                }
                if grid[i][j] != 0 {
                    return false;
                }
            }
        }

        true
    }
}
