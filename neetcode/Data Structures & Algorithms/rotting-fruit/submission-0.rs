use std::collections::VecDeque;

const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let (mut fresh, mut minutes) = (0, 0);
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

        let rows = grid.len();
        let cols = if rows == 0 { 0 } else { grid[0].len() };

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 1 {
                    fresh += 1;
                } else if grid[r][c] == 2 {
                    queue.push_back((r, c));
                }
            }
        }

        while !queue.is_empty() && fresh > 0 {
            minutes += 1;
            for _ in 0..queue.len() {
                let (r, c) = queue.pop_front().unwrap();
                let (r, c) = (r as i32, c as i32);

                for (dr, dc) in DIRS {
                    let nr = r + dr;
                    let nc = c + dc;

                    if 0 <= nr && 0 <= nc && nr < rows as i32 && nc < cols as i32 {
                        let (nr, nc) = (nr as usize, nc as usize);
                        if grid[nr][nc] == 1 {
                            fresh -= 1;
                            grid[nr][nc] = 2;
                            queue.push_back((nr, nc));
                        }
                    }
                }
            }
        }

        if fresh == 0 { minutes } else { -1 }
    }
}
