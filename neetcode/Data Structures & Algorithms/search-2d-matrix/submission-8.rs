impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (rows, cols) = (matrix.len(), matrix[0].len());
        let (mut l, mut r) = (0_i32, rows as i32 - 1);

        while l <= r {
            let mid = (l + (r - l) / 2) as usize;
            let (lower, upper) = (matrix[mid][0], matrix[mid][cols - 1]);

            if lower == target || upper == target {
                return true;
            } else if lower < target && target < upper {
                return Self::binary_search(&matrix[mid], target);
            } else if target < lower {
                r = mid as i32 - 1;
            } else {
                l = mid as i32 + 1;
            }
        }

        false
    }

    fn binary_search(row: &[i32], target: i32) -> bool {
        let (mut l, mut r) = (0, row.len() - 1);

        while l < r {
            let mid = l + (r - l) / 2;
            let cur = row[mid];
            if cur == target {
                return true;
            } else if cur > target {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        row[l] == target
    }
}
