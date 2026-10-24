use std::collections::VecDeque;

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let grid: Vec<&[u8]> = grid.iter().map(|s| s.as_bytes()).collect();
        let (rows, cols) = (grid.len(), grid[0].len());

        let mut start = (0usize, 0usize);
        let mut target_mask = 0usize;

        for (r, row) in grid.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                match cell {
                    b'@' => start = (r, c),
                    b'a'..=b'f' => target_mask |= 1 << (cell - b'a'),
                    _ => {}
                }
            }
        }

        if target_mask == 0 {
            return 0;
        }

        const STATES: usize = 1 << 6;
        let mut visited = vec![vec![[false; STATES]; cols]; rows];

        let mut q = VecDeque::new();
        q.push_back((start.0, start.1, 0usize));
        visited[start.0][start.1][0] = true;

        let mut dist = 0;

        while !q.is_empty() {
            for _ in 0..q.len() {
                let (r, c, mask) = q.pop_front().unwrap();
                if mask == target_mask {
                    return dist;
                }

                for (dr, dc) in DIRS {
                    let (nr, nc) = (r as i32 + dr, c as i32 + dc);
                    if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                        continue;
                    }
                    let (nr, nc) = (nr as usize, nc as usize);
                    let cell = grid[nr][nc];
                    if cell == b'#' {
                        continue;
                    }

                    let mut next_mask = mask;
                    match cell {
                        b'A'..=b'F' => {
                            if mask & (1 << (cell - b'A')) == 0 {
                                continue;
                            }
                        }
                        b'a'..=b'f' => next_mask |= 1 << (cell - b'a'),
                        _ => {}
                    }

                    if visited[nr][nc][next_mask] {
                        continue;
                    }
                    visited[nr][nc][next_mask] = true;
                    q.push_back((nr, nc, next_mask));
                }
            }
            dist += 1;
        }
        
        -1
    }
}
