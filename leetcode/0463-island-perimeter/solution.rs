const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut peri = 0;

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 1 {
                    peri += 4 - Self::count_adj_land(&grid, r, c);
                }
            }
        }

        peri
    }

    fn count_adj_land(grid: &[Vec<i32>], r: usize, c: usize) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut cnt = 0;
        
        for (dr, dc) in DIRS {
            let (nr, nc) = (r as i32 + dr, c as i32 + dc);
            if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                continue;
            }
            let (ur, uc) = (nr as usize, nc as usize);
            cnt += grid[ur][uc];
        }
        
        cnt
    }
}
