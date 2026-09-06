impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (rows, cols) = (matrix.len(), matrix[0].len());
        let mut rseen = vec![false; rows];
        let mut cseen = vec![false; cols];

        for r in 0..rows {
            for c in 0..cols {
                if matrix[r][c] == 0 {
                    rseen[r] = true;
                    cseen[c] = true;
                }
            }
        }

        for r in 0..rows {
            for c in 0..cols {
                if rseen[r] || cseen[c] {
                    matrix[r][c] = 0;
                }
            }
        }
    }
}
