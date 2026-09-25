impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let n = piles.len();
        let mut dp = piles.clone();

        for length in 2..=n {
            for i in 0..=(n - length) {
                let j = i + length - 1;
                dp[i] = (piles[i] - dp[i + 1]).max(piles[j] - dp[i]);
            }
        }

        dp[0] > 0
    }
}
