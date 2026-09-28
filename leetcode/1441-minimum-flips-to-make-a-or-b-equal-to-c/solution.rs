impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut res = 0;
        for i in 0..32 {
            let bit = c & (1 << i);

            // if after-or BIT is 0 => both equivalent bit in a and b must be 0
            if bit == 0 {
                if a & (1 << i) != 0 {
                    res += 1;
                }
                if b & (1 << i) != 0 {
                    res += 1;
                }
            // if after-or BIT is 1 => either bit in a or b must be 1
            } else {
                if a & (1 << i) | b & (1 << i) != (1 << i) {
                    res += 1;
                }
            }
        }
        res
    }
}
