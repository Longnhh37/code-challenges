impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut out = Vec::new();

        let (mut top, mut bottom) = (0, matrix.len() - 1);
        let (mut left, mut right) = (0, matrix[0].len() - 1);
        let done = (bottom + 1) * (right + 1); 

        while left <= right && top <= bottom {
            out.extend_from_slice(&matrix[top][left..=right]);
            top += 1;

            for i in top..=bottom {
                out.push(matrix[i][right]);
            }
            right -= 1;
            
            if out.len() == done {
                break;
            }

            let mut tmp = matrix[bottom][left..=right].to_vec();
            tmp.reverse();
            out.extend_from_slice(&tmp);
            bottom -= 1;

            for j in (top..=bottom).rev() {
                out.push(matrix[j][left]);
            }
            left += 1;
        }

        out
    }
}

