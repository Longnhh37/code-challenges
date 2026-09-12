use std::collections::VecDeque;

const DIRS: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1),
                                (0, -1),          (0, 1),
                                (1, -1), (1, 0),  (1, 1),
                            ];

impl Solution {
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if grid[0][0] == 1 || grid[n - 1][n - 1] == 1 {
            return -1;
        }

        let mut res = 0;
        let mut q = VecDeque::new();
        q.push_back((0, 0));

        while !q.is_empty() {
            res += 1;
            for _ in 0..q.len() {
                let (r, c) = q.pop_front().unwrap();
                grid[r][c] = 1;

                for (dr, dc) in DIRS {
                    let (nr, nc) = (r as i32 + dr, c as i32 + dc);
                    if nr < 0 || nr >= n as i32 || nc < 0 || nc >= n as i32 {
                        continue;
                    }
                    let (ur, uc) = (nr as usize, nc as usize);
                    if ur == n - 1 && uc == n - 1 {
                        return res + 1;
                    }
                    if grid[ur][uc] != 1 {
                        grid[ur][uc] = 1  ;
                        q.push_back((ur, uc));
                    }
                }
            }
        }

        if grid[n - 1][n - 1] == 1 { res } else { -1 }
    }
}
