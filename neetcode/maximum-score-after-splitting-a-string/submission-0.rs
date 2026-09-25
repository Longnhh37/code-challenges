impl Solution {
    pub fn max_score(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut zero = 0i32;
        let mut one = bytes.iter().filter(|&&b| b == b'1').count() as i32;
        let mut res = -1i32;

        for i in 0..bytes.len() - 1 {
            if bytes[i] == b'0' {
                zero += 1;
            } else {
                one -= 1;
            }
            res = res.max(zero + one);
        }

        res
    }
}