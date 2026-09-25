impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let inf = amount + 1;
        let mut dp = vec![inf; amount + 1];
        dp[0] = 0;

        for a in 1..=amount {
            for &coin in &coins {
                let coin = coin as usize;
                if coin <= a  {
                    dp[a] = dp[a].min(1 + dp[a - coin]);
                }
            }
        }

        if dp[amount] <= amount { dp[amount] as i32 } else { -1 }


    }
}
