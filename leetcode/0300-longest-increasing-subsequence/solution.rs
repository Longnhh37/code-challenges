impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len() + 1];

        for (i, &n) in nums.iter().enumerate() {
            for j in 0..i {
                if nums[j] < nums[i] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }

        *dp.iter().max().unwrap()
    }
}
