use std::collections::HashSet;

impl Solution {
    pub fn count_servers(mut grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut res = 0;

        let mut row_checks = vec![false; rows];
        let mut col_checks = vec![false; cols];

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 1 {
                    grid[r][c] = 0;
                    let mut cnt = 0;
                    for i in r + 1..rows {
                        if grid[i][c] == 1 {
                            cnt += 1;
                            grid[i][c] = 0;
                            row_checks[i] = true;
                        }
                    }
                    for j in c + 1..cols {
                        if grid[r][j] == 1 {
                            cnt += 1;
                            grid[r][j] = 0;
                            col_checks[j] = true;
                        }
                    }
                    if cnt > 0 {
                        res += cnt + 1;
                    } else {
                        if row_checks[r] || col_checks[c] {
                            res += 1;
                        }
                    }
                } 
            }
        }

        res
    }
}
