const DIRS: [(isize, isize); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

impl Solution {
    pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut res = 0;

        for r in 0..rows {
            for c in 0..cols {
                res = res.max(Self::dfs(&mut grid, r, c));
            }
        }

        res
    }

    fn dfs(grid: &mut [Vec<i32>], r: usize, c: usize) -> i32 {
        let gold = grid[r][c];
        if gold == 0 {
            return 0;
        }
        grid[r][c] = 0;
        let mut best = 0;

        for (dr, dc) in DIRS {
            let (Some(nr), Some(nc)) = (r.checked_add_signed(dr), c.checked_add_signed(dc)) else {
                continue;
            };
            if nr < grid.len() && nc < grid[0].len() {
                best = best.max(Self::dfs(grid, nr, nc));
            }
        }

        grid[r][c] = gold;
        gold + best
    }
}
