impl Solution {
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (rows, cols) = (mat.len(), mat[0].len());
        let (mut left, mut right) = (0usize, cols - 1);
        while left <= right {
            let mid = left.midpoint(right);

            let max_row = (0..rows)
                .max_by_key(|&r| mat[r][mid])
                .unwrap();
            
            let val = mat[max_row][mid];
            let left_val = mid
                .checked_sub(1)
                .map(|c| mat[max_row][c])
                .unwrap_or(-1);
            let right_val = mat[max_row].get(mid + 1).copied().unwrap_or(-1);

            if val > left_val && val > right_val {
                return vec![max_row as i32, mid as i32];
            } else if left_val > val {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        
        vec![-1, -1]
    }
}
