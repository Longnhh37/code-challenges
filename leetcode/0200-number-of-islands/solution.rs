const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut cnt = 0;
        let (rows, cols) = (grid.len(), grid[0].len());
        let (irows, icols) = (rows as i32, cols as i32);

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '1' {
                    grid[r][c] = '0';
                    cnt += 1;
                    Self::dfs(&mut grid, r, c, irows, icols);
                }
            }
        }

        cnt
    }

    fn dfs(grid: &mut [Vec<char>], r: usize, c: usize, irows: i32, icols: i32) {
        for (dr, dc) in DIRS {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;

            if 0 <= nr && nr < irows && 0 <= nc && nc < icols {
                let (ur, uc) = (nr as usize, nc as usize);
                if grid[ur][uc] == '1' {
                    grid[ur][uc] = '0';
                    Self::dfs(grid, ur, uc, irows, icols);
                }
            }
        }
    }
}
