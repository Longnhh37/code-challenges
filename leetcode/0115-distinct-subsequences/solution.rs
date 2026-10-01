impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let (m, n) = (s.len(), t.len());

        if m < n {
            return 0;
        }

        let mut dp = vec![0u64; n + 1];
        dp[0] = 1;

        for &c in s {
            for j in (1..=n).rev() {
                if c == t[j - 1] {
                    dp[j] += dp[j - 1];
                }
            }
        }

        dp[n] as i32
    }
}
