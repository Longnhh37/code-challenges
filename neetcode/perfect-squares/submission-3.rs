impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;

        for i in 1..=n {
            let mut k = 1;
            while k * k <= i {
                dp[i] = dp[i].min(dp[i - k * k] + 1);
                k += 1;
            }
        }
        dp[n]
    }

}
