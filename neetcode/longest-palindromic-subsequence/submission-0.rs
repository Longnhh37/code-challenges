impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s1: Vec<u8> = s.bytes().rev().collect();
        let s2: Vec<u8> = s.into_bytes();
        let (m, n) = (s1.len(), s2.len());
        let mut dp = vec![vec![0i32; m + 1]; n + 1];

        for i in 0..n {
            for j in 0..m {
                if s1[i] == s2[j] {
                    dp[i + 1][j + 1] = 1 + dp[i][j];
                } else {
                    dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
                }
            }
        }

        dp[n][m]
    }
}