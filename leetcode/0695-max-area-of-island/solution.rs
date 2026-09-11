use std::collections::VecDeque;

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let (irows, icols) = (rows as i32, cols as i32);

        let mut largest = 0;
        let mut q = VecDeque::new();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] != 1 {
                    continue;
                }
                grid[r][c] = 0;
                let mut cur = 0;
                q.push_back((r, c));

                while let Some((r, c)) = q.pop_front() {
                    cur += 1;
                    for (dr, dc) in DIRS {
                        let (nr, nc) = (r as i32 + dr, c as i32 + dc);
                        if nr < 0 || nr >= irows || nc < 0 || nc >= icols {
                            continue;
                        }
                        let (ur, uc) = (nr as usize, nc as usize);
                        if grid[ur][uc] == 1 {
                            grid[ur][uc] = 0;
                            q.push_back((ur, uc));
                        }
                    }
                }
                largest = largest.max(cur);
            }
        }

        largest
    }
}

