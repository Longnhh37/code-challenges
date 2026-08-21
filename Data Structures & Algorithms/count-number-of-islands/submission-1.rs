use std::collections::VecDeque;

const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut count = 0;
        let mut queue = VecDeque::new();

        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == '0' {
                    continue;
                }
                count += 1;
                grid[i][j] = '0';
                queue.push_back((i, j));

                while let Some((ci, cj)) = queue.pop_front() {
                    let ci = ci as i32;
                    let cj = cj as i32;
                    for (di, dj) in DIRS {
                        let ni = ci + di;
                        let nj = cj + dj;
                        if ni < 0 
                            || ni >= rows as i32
                            || nj < 0 
                            || nj >= cols as i32 {
                                continue;
                        }
                        let ni = ni as usize;
                        let nj = nj as usize;
                        if grid[ni][nj] == '1' {
                            grid[ni][nj] = '0';
                            queue.push_back((ni, nj));
                        }
                    }
                }
            }
        }

        count
    }
}
