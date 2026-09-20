impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![0i32; n];

        for i in (0..n).rev() {
            let mut prev = 0;
            dp[i] = 1;
            for j in i + 1..n {
                let old = dp[j];
                dp[j] = if s[i] == s[j] {
                    prev + 2
                } else {
                    dp[j].max(dp[j - 1])
                };
                prev = old;
            }
        }

        dp[n - 1]
    }
}
