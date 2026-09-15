impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (rows, cols) = (matrix.len(), matrix[0].len());

        let (mut r, mut c) = (0i32, cols as i32 - 1);

        while r < rows as i32 && c >= 0 {
            let val = matrix[r as usize][c as usize];
            if val == target {
                return true;
            } else if val > target {
                c -= 1;
            } else {
                r += 1;
            }
        }

        false
    }
}
