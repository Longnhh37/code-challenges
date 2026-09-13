impl Solution {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        cost.push(0);
        let n = cost.len();
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;
        dp[1] = 0;

        for i in 2..n + 1 {
            dp[i] = (dp[i - 2] + cost[i - 2]).min(dp[i - 1] + cost[i - 1]);
        }

        dp[n]
    }
}
