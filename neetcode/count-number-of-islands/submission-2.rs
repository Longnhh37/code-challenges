const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut res = 0;
        let (rows, cols) = (grid.len(), grid[0].len());
        let (irows, icols) = (rows as i32, cols as i32);

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '0' {
                    continue;
                }
                grid[r][c] = '0';
                res += 1;
                Self::dfs(&mut grid, irows, icols, r, c);
            }
        }

        res
    }

    fn dfs(grid: &mut Vec<Vec<char>>, rows: i32, cols: i32, r: usize, c: usize) {
        let (ir, ic) = (r as i32, c as i32);
        for (dr, dc) in DIRS {
            let (nr, nc) = (ir + dr, ic + dc);
            if 0 <= nr && nr < rows && 0 <= nc && nc < cols {
                let (ur, uc) = (nr as usize, nc as usize);
                if grid[ur][uc] == '1' {
                    grid[ur][uc] = '0';
                    Self::dfs(grid, rows, cols, ur, uc);
                }
            }
        }
    }
}
