impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut res = vec![vec![0; n]; n];

        let (mut l, mut r) = (0i32, n as i32 - 1);
        let (mut t, mut b) = (0i32, n as i32 - 1);

        let mut val = 1;

        while l <= r {
            for i in l..=r {
                res[t as usize][i as usize] = val;
                val += 1;
            }
            t += 1;

            for i in t..=b {
                res[i as usize][r as usize] = val;
                val += 1;
            }
            r -= 1;

            for i in (l..=r).rev() {
                res[b as usize][i as usize] = val;
                val += 1;
            }
            b -= 1;

            for i in (t..=b).rev() {
                res[i as usize][l as usize] = val;
                val += 1;
            }
            l += 1;
        }

        res
    }
}
