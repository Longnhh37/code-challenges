use std::cmp::min;

impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if n == 1 {
            return grid[0][0];
        }

        fn two_min(row: &[i32]) -> ((i32, usize), (i32, usize)) {
            let mut m1 = (i32::MAX, usize::MAX);
            let mut m2 = (i32::MAX, usize::MAX);
            for (i, &v) in row.iter().enumerate() {
                if v < m1.0 {
                    m2 = m1;
                    m1 = (v, i);
                } else if v < m2.0 {
                    m2 = (v, i);
                }
            }
            (m1, m2)
        }

        let mut prev = grid[0].clone();

        for row in &grid[1..] {
            let (m1, m2) = two_min(&prev);
            for (c, cell) in row.iter().enumerate() {
                let best = if c == m1.1 { m2.0 } else { m1.0 }; 
                prev[c] = cell + best;
            }
        }

        prev.into_iter().min().unwrap()
    }
}
