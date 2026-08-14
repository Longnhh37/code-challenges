use std::collections::HashMap;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[1] = 1;

        for i in 1..=n {
            if Self::is_square(i as i32) {
                dp[i] = 1;
                continue;
            }

            let mut best = i32::MAX;
            for j in 1..=i / 2 {
                best = best.min(dp[j] + dp[i - j]);
            }
            dp[i] = best;
        }
        println!("{:?}", dp);
        dp[n]
    }

    fn is_square(n: i32) -> bool {
        let root = (n as f32).sqrt() as i32;
        root * root == n
    }
}
