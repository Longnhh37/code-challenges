use std::collections::VecDeque;

const DIRS: [(i32, i32); 3] = [(1, 1), (1, 0), (0, 1)];

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][0] == 1 {
            return -1;
        }
        let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
        let mut res = 0;
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));

        while !queue.is_empty() {
            res += 1;
            for _ in 0..queue.len() {
                let (r, c) = queue.pop_front().unwrap();
                let (ir, ic) = (r as i32, c as i32);
                if ir == rows - 1 && ic == cols - 1 {
                    return res;
                }
                for (dr, dc) in DIRS {
                    let (nr, nc) = (ir + dr, ic + dc);
                    if nr < rows && nc < cols {
                        let (ur, uc) = (nr as usize, nc as usize);
                        if grid[ur][uc] == 0 {
                            queue.push_back((nr as usize, nc as usize));
                        }
                    }
                }
            }
        }

        -1
    }
}
