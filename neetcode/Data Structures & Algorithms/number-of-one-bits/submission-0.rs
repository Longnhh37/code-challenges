impl Solution {
    pub fn hamming_weight(mut n: u32) -> i32 {
        let mut res = 0;
        while n > 0 {
            res += n & 1;
            n >>= 1;
        }
        res as i32
    }
}
