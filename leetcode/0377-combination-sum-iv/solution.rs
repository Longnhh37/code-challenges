impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let mut dp = vec![0u32; target + 1];
        dp[0] = 1;

        for t in 1..=target {
            for &n in &nums {
                let n = n as usize;
                if n <= t {
                    dp[t] += dp[t - n];
                }
            }
        }

        dp[target] as i32
    }
}
