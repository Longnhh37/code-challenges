impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return Vec::new();
        }

        let (rows, cols) = (matrix.len(), matrix[0].len());
        let mut res = Vec::with_capacity(rows * cols);

        let (mut top, mut bottom) = (0_isize, rows as isize - 1);
        let (mut left, mut right) = (0_isize, cols as isize - 1);

        while top <= bottom && left <= right {
            for i in left..=right {
                res.push(matrix[top as usize][i as usize]);
            }
            top += 1;

            for i in top..=bottom {
                res.push(matrix[i as usize][right as usize]);
            }
            right -= 1;

            if top <= bottom {
                for i in (left..=right).rev() {
                    res.push(matrix[bottom as usize][i as usize]);
                }
                bottom -= 1;
            }

            if left <= right {
                for i in (top..=bottom).rev() {
                    res.push(matrix[i as usize][left as usize]);
                }
                left += 1;
            }
        }

        res
    }
}
