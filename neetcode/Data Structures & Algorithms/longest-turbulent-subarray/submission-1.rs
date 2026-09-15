impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let n = (n - 1) as i64;
        let x = x as i64;
        let mut res = x;
        let mut n_bit = 0;
        let mut bit_pos = 0i64;

        while (n >> n_bit) > 0 {
            while (res >> bit_pos) & 1 == 1 {
                bit_pos += 1;
            }
            if (n >> n_bit) & 1 == 1 {
                res |= 1i64 << bit_pos;
            }
            bit_pos += 1;
            n_bit += 1;
        }
        res
    }
}

