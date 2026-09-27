impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let (rows, cols) = (matrix.len(), matrix[0].len());
        let mut heights = vec![0i32; cols];
        let mut best = 0;

        for i in 0..rows {
            for j in 0..cols {
                if matrix[i][j] == '1' {
                    heights[j] += 1;
                } else {
                    heights[j] = 0;
                }
            }
            best = best.max(Self::largest_rectangle_area(&heights));
        }

        best
    }

    fn largest_rectangle_area(heights: &[i32]) -> i32 {
        let mut n = heights.len();
        let left = {
            let mut res = vec![-1i32; n];
            let mut stack = Vec::new();
            for i in 0..n {
                while let Some(&top) = stack.last() {
                    if heights[top] >= heights[i] {
                        stack.pop();
                    } else {
                        break;
                    }
                }
                res[i] = stack.last().map_or(-1, |&idx| idx as i32);
                stack.push(i);
            }
            res
        };
        let right = {
            let mut res = vec![n as i32; n];
            let mut stack = Vec::new();
            for i in (0..n).rev() {
                while let Some(&top) = stack.last() {
                    if heights[top] >= heights[i] {
                        stack.pop();
                    } else {
                        break;
                    }
                }
                res[i] = stack.last().map_or(n as i32, |&idx| idx as i32);
                stack.push(i);
            }
            res
        };
    
        (0..n)
            .map(|i| heights[i] * (right[i] - left[i] - 1))
            .max()
            .unwrap_or(0)
    }
}
