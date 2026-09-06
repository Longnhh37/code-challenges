impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut matrix = vec![vec![0; n]; n];

        let (mut t, mut b) = (0isize, n as isize - 1);
        let (mut l, mut r) = (0isize, n as isize - 1);

        let mut cur = 1;

        while t <= b && l <= r {
            for i in l..=r {
                matrix[t as usize][i as usize] = cur;
                cur += 1;
            }
            t += 1;

            for i in t..=b {
                matrix[i as usize][r as usize] = cur;
                cur += 1;
            }
            r -= 1;

            if t <= b {
                for i in (l..=r).rev() {
                    matrix[b as usize][i as usize] = cur;
                    cur += 1;
                }
                b -= 1;
            }

            if l <= r {
                for i in (t..=b).rev() {
                    matrix[i as usize][l as usize] = cur;
                    cur += 1;
                }               
                l += 1;
            }
        }

        matrix
    }
}
