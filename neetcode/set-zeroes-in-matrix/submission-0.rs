use std::collections::HashSet;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (mut zr, mut zc) = (HashSet::new(), HashSet::new());
        for r in 0..matrix.len() {
            for c in 0..matrix[0].len() {
                if matrix[r][c] == 0 {
                    zr.insert(r);
                    zc.insert(c);
                }
            }
        }

        for r in 0..matrix.len() {
            for c in 0..matrix[0].len() {
                if zr.contains(&r) || zc.contains(&c) {
                    matrix[r][c] = 0;
                }
            }
        }
    }
}
