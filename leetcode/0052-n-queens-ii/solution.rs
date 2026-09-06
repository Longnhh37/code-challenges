impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut res = 0;
        Self::dfs(0, n, 0, 0, 0, &mut res);
        res
    }

    fn dfs(
        row: usize,
        n: usize,
        cols: u32,
        diag: u32,
        anti_diag: u32,
        res: &mut i32,
    ) {
        if row == n {
            return *res += 1;
        }

        let full_mask = (1u32 << n) - 1;
        let occupied = cols | diag | anti_diag;
        let mut avail = full_mask & !occupied;

        while avail != 0 {
            let bit = avail & avail.wrapping_neg();
            avail -= bit;
            Self::dfs(row + 1, n , cols | bit, (diag | bit) << 1, (anti_diag | bit) >> 1, res);
        }
    }
}
