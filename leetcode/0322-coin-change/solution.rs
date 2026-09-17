impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amt = amount as usize;
        let mut dp = vec![i32::MAX; amt + 1];
        dp[0] = 0;
        for i in 1..=amt {
            for &c in &coins {
                let c = c as usize;
                if c <= i && dp[i - c] != i32::MAX {
                    dp[i] = dp[i].min(dp[i - c] + 1);
                }
            }
        }

        if dp[amt] != i32::MAX {
            dp[amt]
        } else {
            -1
        }
    }
}
