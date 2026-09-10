impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![0i32; n + 1];
        dp[0] = 1;
        dp[1] = if s[0] != b'0' { 1 } else { 0 };

        for i in 2..=n {
            let one = s[i - 1];
            let two = &s[i-2..i];
            if one != b'0' {
                dp[i] += dp[i - 1];
            }
            let two_val: u32 = std::str::from_utf8(two).unwrap().parse().unwrap();
            if (10..=26).contains(&two_val) {
                dp[i] += dp[i - 2];
            }
        }
        dp[n] as i32
    }
}
