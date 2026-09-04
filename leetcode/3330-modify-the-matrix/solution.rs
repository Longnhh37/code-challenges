impl Solution {
    pub fn modified_matrix(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let cols = matrix[0].len();
        let mut seen = vec![-1; cols];

        for i in 0..matrix.len() {
            for j in 0..cols {
                if matrix[i][j] != -1 {
                    continue;
                }
                if seen[j] != -1 {
                    matrix[i][j] = seen[j];
                } else {
                    let max = Self::get_max(&matrix, j);
                    matrix[i][j] = max;
                    seen[j] = max;
                }
            }
        }

        matrix
    }
    fn get_max(matrix: &[Vec<i32>], c: usize) -> i32 {
        matrix.iter().map(|row| row[c]).max().unwrap()
    }
}
