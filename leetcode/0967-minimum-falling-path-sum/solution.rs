impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let mut prev = matrix[0].clone();

        for r in 1..n {
            let mut cur = vec![0; n];
            for c in 0..n {
                let mut min_d = prev[c];
                if c > 0 {
                    min_d = min_d.min(prev[c - 1]);
                }
                if c < n - 1 {
                    min_d = min_d.min(prev[c + 1]);
                }

                cur[c] = min_d + matrix[r][c];
            }
            prev = cur;
        }

        *prev.iter().min().unwrap()
    }
}
