impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        if n == 1 {
            return vec![vec!["Q".to_string()]];
        }

        let n = n as usize;
        let mut res: Vec<Vec<String>> = Vec::new();
        let mut cur: Vec<Vec<u8>> = vec![vec![b'.'; n]; n];
        Self::dfs(0, n, &mut vec![false; 2 * n], &mut vec![false; 2 * n], &mut vec![false; n], &mut cur, &mut res);
        res
    }

    fn dfs(
        r: usize,
        size: usize,
        diag: &mut Vec<bool>,
        anti_diag: &mut Vec<bool>,
        cols: &mut Vec<bool>,
        cur: &mut Vec<Vec<u8>>,
        res: &mut Vec<Vec<String>>,
    ) {
        if r == size {
            let temp: Vec<String> = cur
                .iter()
                .map(|item| String::from_utf8(item.clone()).unwrap())
                .collect();
            res.push(temp);
            return;
        }

        for c in 0..size {
            let cur_diag = r - c + size - 1;
            let cur_anti_diag = r + c;

            if cols[c] || diag[cur_diag] || anti_diag[cur_anti_diag] {
                continue;
            }

            cur[r][c] = b'Q';
            cols[c] = true;
            diag[cur_diag] = true;
            anti_diag[cur_anti_diag] = true;

            Self::dfs(r + 1, size, diag, anti_diag, cols, cur, res);

            cur[r][c] = b'.';
            cols[c] = false;
            diag[cur_diag] = false;
            anti_diag[cur_anti_diag] = false;
            
        }

    }
}
