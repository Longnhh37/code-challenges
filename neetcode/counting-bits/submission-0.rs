impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut dp = vec![0_i32; n as usize + 1];
        for i in 1..=n {
            let i = i as usize;
            dp[i] = dp[i >> 1] + (i & 1) as i32;
        }
        dp
    }
}
