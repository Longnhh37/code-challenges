use std::collections::VecDeque;

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let (irows, icols) = (rows as i32, cols as i32);

        let mut res = -1;
        let mut fresh_exist = false;
        let mut q = VecDeque::new();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 2 {
                    q.push_back((r, c));
                } else if grid[r][c] == 1 {
                    fresh_exist = true;
                }
            }
        }

        if q.is_empty() {
            if fresh_exist {
               return -1;
            } else {
                return 0;
            }
        }

        while !q.is_empty() {
            res += 1;
            for _ in 0..q.len() {
                let (r, c) = q.pop_front().unwrap();
                for (dr, dc) in DIRS {
                    let (nr, nc) = (r as i32 + dr, c as i32 + dc);
                    if 0 <= nr && nr < irows && 0 <= nc && nc < icols {
                        let (ur, uc) = (nr as usize, nc as usize);
                        if grid[ur][uc] != 1 {
                            continue;
                        }
                        grid[ur][uc] = 2;
                        q.push_back((ur, uc));
                    }
                }
            }
        }

        if grid.iter().all(|row| row.iter().all(|&cell| cell != 1)) {
            res
        } else {
            -1
        }
    }
}
