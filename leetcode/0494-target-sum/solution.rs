impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let total: i32 = nums.iter().sum();

        if target.abs() > total || (total + target) & 1 != 0 {
            return 0;
        }

        let goal = ((target + total) / 2) as usize;
        let mut dp = vec![0i32; goal + 1];
        dp[0] = 1;

        for &n in &nums {
            let n = n as usize;
            for s in (n..=goal).rev() {
                dp[s] += dp[s - n];
            }
        }
        
        dp[goal]
    }
}

