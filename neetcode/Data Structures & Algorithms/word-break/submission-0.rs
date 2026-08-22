impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        let sb = s.as_bytes();
        let mut dp = vec![false; n + 1];
        dp[n] = true;

        for start in (0..n).rev() {
            for w in &word_dict {
                let wb = w.as_bytes();
                let end = start + wb.len();
                if end <= n && &sb[start..end] == wb {
                    dp[start] = dp[end];
                }
                if dp[start] {
                    break;
                }
            }
        }
        dp[0]
    }
}
