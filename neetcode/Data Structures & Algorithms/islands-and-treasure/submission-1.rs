use std::collections::VecDeque;

const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const LAND: i32 = i32::MAX;

impl Solution {
    pub fn islands_and_treasure(grid: &mut Vec<Vec<i32>>) {
        let mut q = VecDeque::new();
        let (rows, cols) = (grid.len(), grid[0].len());

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 0 {
                    q.push_back((r, c));
                }
            }
        }
        if q.is_empty() {
            return;
        }

        while let Some((r, c)) = q.pop_front() {
            for (dr, dc) in DIRS {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr < 0 || nc < 0 || nr >= rows as i32 || nc >= cols as i32 {
                    continue;
                }
                let (ur, uc) = (nr as usize, nc as usize);
                if grid[ur][uc] != LAND {
                    continue;
                }
                grid[ur][uc] = grid[r][c] + 1;
                q.push_back((ur, uc));
            }
        }
    }
}
