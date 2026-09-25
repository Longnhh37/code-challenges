impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::new();
        let (mut left, mut right) = (0, matrix[0].len() - 1);
        let (mut top, mut bottom) = (0, matrix.len() - 1);

        while top <= bottom && left <= right {
            for i in left..=right {
                res.push(matrix[top][i]);
            }
            top += 1;

            for i in top..=bottom {
                res.push(matrix[i][right]);
            }
            match right.checked_sub(1) {
                Some(v) => right = v,
                None => return res,
            }

            if top <= bottom {
                for i in (left..=right).rev() {
                    res.push(matrix[bottom][i]);
                }
                match bottom.checked_sub(1) {
                    Some(v) => bottom = v,
                    None => return res,
                }
            }

            if left <= right {
                for i in (top..=bottom).rev() {
                    res.push(matrix[i][left]);
                }
                left += 1;
            }
        }

        res
    }
}
