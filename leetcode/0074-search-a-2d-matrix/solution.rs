impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (rows, cols) = (matrix.len(), matrix[0].len());

        if rows == 1 {
            return matrix[0].binary_search(&target).is_ok();
        }

        let (mut top, mut btm) = (0, rows - 1);
        while top < btm {
            let (l1, r1) = (matrix[top][0], matrix[top][cols - 1]);
            let (l2, r2) = (matrix[btm][0], matrix[btm][cols - 1]);

            if l1 <= target && target <= r1 {
                return matrix[top].binary_search(&target).is_ok();
            } else if l2 <= target && target <= r2 {
                return matrix[btm].binary_search(&target).is_ok();
            } else {
                let mid = top + (btm - top + 1) / 2;
                if target < matrix[mid][0] {
                    btm = mid - 1;
                } else {
                    top = mid;
                }
            }
        }

        false
    }
}
