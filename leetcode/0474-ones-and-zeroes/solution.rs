impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let (n, m) = (n as usize, m as usize);
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for s in &strs {
            let ones = s.bytes().filter(|&b| b == b'1').count();
            let zeros = s.len() - ones;

            if ones > n || zeros > m {
                continue;
            }

            for i in (zeros..=m).rev() {
                for j in (ones..=n).rev() {
                    dp[i][j] = dp[i][j].max(dp[i - zeros][j - ones] + 1);
                }
            }
        }

        dp[m][n] as i32
    }
}
