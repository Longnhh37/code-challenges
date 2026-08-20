impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut cols = vec![false; n];
        let mut diag = vec![false; 2 * n - 1];
        let mut anti_diag = vec![false; 2 * n - 1];
        let mut count = 0;

        Self::dfs(0, n, &mut cols, &mut diag, &mut anti_diag, &mut count);
        count
    }

    fn dfs(row: usize, n: usize, cols: &mut [bool], diag: &mut [bool], anti_diag: &mut [bool], count: &mut i32) {
        if row == n {
            *count += 1;
            return;
        }

        for col in 0..n {
            let d = row + n - 1 - col;
            let ad = row + col;

            if cols[col] || diag[d] || anti_diag[ad] {
                continue;
            }

            cols[col] = true;
            diag[d] = true;
            anti_diag[ad] = true;

            Self::dfs(row + 1, n, cols, diag, anti_diag, count);

            cols[col] = false;
            diag[d] = false;
            anti_diag[ad] = false;
        }
    }
}
