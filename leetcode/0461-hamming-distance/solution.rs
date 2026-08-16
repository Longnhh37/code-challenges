impl Solution {
    pub fn hamming_distance(mut x: i32, mut y: i32) -> i32 {
        let mut res = 0;
        while x > 0 || y > 0 {
            res += (x & 1) ^ (y & 1);
            x >>= 1;
            y >>= 1;
        }
        res
    }
}
