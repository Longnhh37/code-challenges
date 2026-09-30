impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (matrix.len(), matrix[0].len());
        let mut prev = matrix[0].clone();
        let mut cur = vec![0; cols];

        for r in 1..rows {
            for c in 0..cols {
                cur[c] = prev[c];
                if c > 0 {
                    cur[c] = cur[c].min(prev[c - 1]);
                }
                if c < cols - 1 {
                    cur[c] = cur[c].min(prev[c + 1]);
                }
                cur[c] += matrix[r][c];
            }
            std::mem::swap(&mut prev, &mut cur);
        }

        prev.into_iter().min().unwrap()
    }
}
