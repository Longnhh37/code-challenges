impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut k = 0i32;
        while let Some(mul) = k.checked_mul(k) && mul <= c {
            if Self::is_square(mul) && Self::is_square(c - mul) {
                return true;
            }
            k += 1;
        }
        false
    }

    fn is_square(n: i32) -> bool {
        let root = (n as f64).sqrt() as i32;
        root * root == n
    }
}
