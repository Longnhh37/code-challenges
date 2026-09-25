impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let mut res = 0;

        for i in 0..n {
            res += mat[i][i];
        }
        
        for i in 0..n {
            res += mat[n - i - 1][i];
        }

        if n % 2 == 1 {
            let i = n / 2;
            res -= mat[i][i];
        }
        
        res
    }
}
