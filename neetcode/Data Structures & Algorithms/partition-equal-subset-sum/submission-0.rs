impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total = nums.iter().sum::<i32>();
        if total % 2 == 1 {
            return false;
        }
        let target = (total / 2) as usize;
        let mut dp = vec![false; target + 1];
        dp[0] = true;

        for &n in &nums {
            let n = n as usize;
            for s in (n..=target).rev() {
                if dp[s - n] {
                    dp[s] = true;
                }
            }
            if dp[target] {
                return true;
            }
        }

        dp[target]
    }
}
