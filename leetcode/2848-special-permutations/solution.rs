impl Solution {
    pub fn special_perm(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let n = nums.len();
        let size = 1usize << n;

        let mut dp = vec![vec![0i64; n]; size];
        for i in 0..n {
            dp[1usize << i][i] = 1;
        }

        for mask in 1..size {
            for i in 0..n {
                if mask & (1usize << i) == 0 {
                    continue;
                }
                let ways = dp[mask][i];
                if ways == 0 {
                    continue;
                }

                for j in 0..n {
                    let bit = 1usize << j;
                    if mask & bit != 0 {
                        continue;
                    }
                    if nums[i] % nums[j] != 0 && nums[j] % nums[i] != 0 {
                        continue;
                    }

                    let next_mask = mask | bit;
                    dp[next_mask][j] = (dp[next_mask][j] + ways) % MOD;
                }
            }
        }

        let full = size - 1;
        let mut ans = 0i64;

        for i in 0..n {
            ans = (ans + dp[full][i]) % MOD;
        }

        ans as i32
    }
}

