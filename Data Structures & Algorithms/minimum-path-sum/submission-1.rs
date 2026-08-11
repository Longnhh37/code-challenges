impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut prev: Vec<i32> = grid[0].iter().scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect();

        for r in 1..rows {
            let mut cur = vec![0; cols];
            cur[0] = prev[0] + grid[r][0];
            for c in 1..cols {
                cur[c] = grid[r][c] + cur[c - 1].min(prev[c]);
            }
            prev = cur;
        }

        prev[cols - 1]
    }
}
