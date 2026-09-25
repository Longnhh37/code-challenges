const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut largest = 0;

        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == 1 {
                    let mut count = 0;
                    Self::dfs(&mut grid, i as i32, j as i32, rows as i32, cols as i32, &mut count);
                    largest = largest.max(count);
                }
            }
        }

        largest
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32, rows: i32, cols: i32, count: &mut i32) {
        if i < 0 || i >= rows || j < 0 || j >= cols {
            return;
        }
        let (ui, uj) = (i as usize, j as usize);
        if grid[ui][uj] != 1 {
            return;
        }
        grid[ui][uj] = 0;
        *count += 1;

        for (di, dj) in DIRS {
            Self::dfs(grid, i + di, j + dj, rows, cols, count);
        }

    }
}
