use std::collections::VecDeque;

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

impl Solution {
    pub fn find_safe_walk(mut grid: Vec<Vec<i32>>, health: i32) -> bool {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut dist = vec![vec![i32::MAX; cols]; rows];
        let mut q = VecDeque::new();
        dist[0][0]= grid[0][0];
        q.push_back((0usize, 0usize));

        while let Some((r, c)) = q.pop_front() {
            let d = dist[r][c];
            for (dr, dc) in DIRS {
                let (nr, nc) = (r as i32 + dr, c as i32 + dc);
                if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                    continue;
                }
                let (ur, uc) = (nr as usize, nc as usize);
                let w = grid[ur][uc];
                if d + w < dist[ur][uc] {
                    dist[ur][uc] = d + w;
                    if w == 0 {
                        q.push_front((ur, uc));
                    } else {
                        q.push_back((ur, uc));
                    }
                }
            }
        }

        dist[rows - 1][cols - 1] < health
    }
}

