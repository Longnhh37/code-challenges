impl Solution {
    pub fn unique_paths_with_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0][0] == 1 || *grid.last().unwrap().last().unwrap() == 1 {
            return 0;
        }

        let cols = grid[0].len();
        let mut prev = vec![1; cols];
        for c in 0..cols {
            if grid[0][c] == 1 {
                for i in c..cols {
                    prev[i] = 0;
                }
                break;
            }
        }

        for r in 1..grid.len() {
            let mut cur = vec![0; cols];
            if grid[r][0] == 0 {
                cur[0] = prev[0];
            }
            for c in 1..cols {
                if grid[r][c] == 1 {
                    continue;
                }
                cur[c] += prev[c] + cur[c - 1];
            }
            prev = cur;
        }
        
        prev[cols - 1]
    }
}
