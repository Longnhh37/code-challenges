impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let b = s.as_bytes();
        let n = b.len();
        let mut res = -1i32;

        for l in 0..n {
            for r in (l..n).rev() {
                if b[l] == b[r] {
                    res = res.max((r - l - 1) as i32);
                }
            }
        }
        
        res
    }
}