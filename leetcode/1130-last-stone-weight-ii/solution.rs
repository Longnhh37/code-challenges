impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum: i32 = stones.iter().sum();
        let target = sum / 2;

        let mut dp = vec![false; (target + 1) as usize];
        dp[0] = true;

        for st in stones {
            let st = st as usize;
            for j in (st..=target as usize).rev() {
                if dp[j - st] {
                    dp[j] = true;
                }
            }
        }

        let best = (0..=target as usize)
            .rev()
            .find(|&j| dp[j])
            .unwrap_or(0) as i32;

        sum - 2 * best
    }
}
