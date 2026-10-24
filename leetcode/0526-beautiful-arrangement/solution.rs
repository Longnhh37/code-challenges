impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let n = n as usize;
        let size = 1usize << n;

        let mut dp = vec![0i32; size];
        dp[0] = 1;

        for mask in 0..size {
            let pos = mask.count_ones() as usize + 1;
            if pos > n {
                continue;
            }
            for x in 1..=n {
                let bit = 1usize << (x - 1);
                if mask & bit != 0 {
                    continue;
                }
                if x % pos != 0 && pos % x != 0 {
                    continue;
                }
                dp[mask | bit] += dp[mask];
            }
        }

        dp[size - 1]
    }
}
