impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut res = 0;
        Self::backtrack(0, n, 0, 0, 0, &mut res);
        res
    }

    fn backtrack(
        row: i32,
        n: i32,
        cols: u32,
        diag1: u32, // row + n - col
        diag2: u32, // row + col
        res: &mut i32,
    ) {
        if row == n {
            return *res += 1;
        }

        let full_mask = (1u32 << n) - 1;
        let occupied = cols | diag1 | diag2;
        let mut avail = full_mask & !occupied;

        while avail != 0 {
            let bit = avail & avail.wrapping_neg();
            avail -= bit;
            Self::backtrack(
                row + 1,
                n,
                cols | bit,
                (diag1 | bit) << 1,
                (diag2 | bit) >> 1,
                res
            );
        }
    }
}
