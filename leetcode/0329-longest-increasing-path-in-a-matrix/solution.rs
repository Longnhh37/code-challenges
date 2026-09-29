use std::collections::VecDeque;

static DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (matrix.len(), matrix[0].len());
        
        let mut in_deg = vec![vec![0u32; cols]; rows];
        for r in 0..rows {
            for c in 0..cols {
                for (ur, uc) in Self::neighbors(&DIRS, rows as i32, cols as i32, r, c) {
                    if matrix[ur][uc] < matrix[r][c] {
                        in_deg[r][c] += 1;
                    }
                }
            }
        }

        let mut q = VecDeque::new();
        for r in 0..rows {
            for c in 0..cols {
                if in_deg[r][c] == 0 {
                    q.push_back((r, c));
                }
            }
        }

        let mut levels = 0;
        while !q.is_empty() {
            levels += 1;
            for _ in 0..q.len() {
                let (r, c) = q.pop_front().unwrap();
                for (ur, uc) in Self::neighbors(&DIRS, rows as i32, cols as i32, r, c) {
                    if matrix[ur][uc] > matrix[r][c] {
                        in_deg[ur][uc] -= 1;
                        if in_deg[ur][uc] == 0 {
                            q.push_back((ur, uc));
                        }
                    }
                }
            }
        }

        levels
    }

    fn neighbors(dirs: &'static [(i32, i32); 4], rows: i32, cols: i32, r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut res = Vec::new();

        for (dr, dc) in dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if 0 <= nr && nr < rows && 0 <= nc && nc < cols {
                res.push((nr as usize, nc as usize));
            }
        }

        res
    }
}
