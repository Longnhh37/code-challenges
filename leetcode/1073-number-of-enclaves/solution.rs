const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

impl Solution {
    pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &mut [Vec<i32>], r: usize, c: usize) { 
            let (rows, cols) = (grid.len(), grid[0].len());
            for (dr, dc) in DIRS {
                let (nr, nc) = (r as i32 + dr, c as i32 + dc);
                if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                    continue;
                }
                let (ur, uc) = (nr as usize, nc as usize);
                if grid[ur][uc] == 1 {
                    grid[ur][uc] = 0;
                    dfs(grid, ur, uc);
                }
            }
        }

        let (rows, cols) = (grid.len(), grid[0].len());
        for r in 0..rows {
            for c in 0..cols {
                let on_border = r == 0 || r == rows - 1 || c == 0 || c == cols - 1;
                if on_border && grid[r][c] == 1 {
                    grid[r][c] = 0;
                    dfs(&mut grid, r, c);
                }
            }
        }

        let mut res = 0;
        for r in 0..rows {
            for c in 0..cols {
                res += grid[r][c];
            }
        }

        res
    }
}
