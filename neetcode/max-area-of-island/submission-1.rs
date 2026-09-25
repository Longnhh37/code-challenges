const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
        let mut largest = 0;
        let mut stack = Vec::new();

        for i in 0..rows {
            for j in 0..cols {
                if grid[i as usize][j as usize] != 1 {
                    continue;
                }
                let mut count = 0;
                grid[i as usize][j as usize] = 0;
                stack.push((i, j));

                while let Some((ci, cj)) = stack.pop() {
                    count += 1;
                    for (di, dj) in DIRS {
                        let ni = ci + di;
                        let nj = cj + dj;
                        if ni < 0 || ni >= rows || nj < 0 || nj >= cols {
                            continue;
                        }
                        let (uni, unj) = (ni as usize, nj as usize);
                        if grid[uni][unj] == 1 {
                            grid[uni][unj] = 0;
                            stack.push((ni, nj));
                        }
                    }
                }
                largest = largest.max(count);
            }
        }

        largest
    }
}
