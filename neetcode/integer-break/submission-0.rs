impl Solution {
    pub fn integer_break(num: i32) -> i32 {
        if num == 2 {
            return 1;
        } else if num == 3 {
            return 2;
        }

        let num = num as usize;
        let mut dp = vec![0usize; num + 1];
        dp[2] = 2;
        dp[3] = 3;

        for n in 4..=num {
            let mut max_prod = n - 1;
            for i in 2..n {
                max_prod = max_prod.max(dp[i] * (n - i));
            }
            dp[n] = max_prod;
        }

        dp[num] as i32
    }
}
