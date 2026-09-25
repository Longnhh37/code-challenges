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
                queue.push_back((i, j));

                while !queue.is_empty() {
                    let (i, j) = queue.pop_front().unwrap();
                    let i = i as i32;
                    let j = j as i32;

                    for (di, dj) in DIRS {
                        let ni = i + di;
                        let nj = j + dj;
                        if ni < 0 
                            || ni >= rows as i32
                            || nj < 0 
                            || nj >= cols as i32 {
                                continue;
                        }
                        let ni = ni as usize;
                        let nj = nj as usize;
                        if grid[ni][nj] == '0' {
                            continue;
                        }
                        queue.push_back((ni, nj));
                        grid[ni][nj] = '0';
                    }
                }
            }
        }

        count
    }
}
