impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        for (i, &n) in prices.iter().enumerate() {
            match Self::next_cheaper(&prices, i, n) {
                None => res.push(n),
                Some(j) => res.push(n - prices[i + j + 1]),
            }
        }

        res
    }

    fn next_cheaper(prices: &[i32], start: usize, target: i32) -> Option<usize> {
        prices[start + 1..].iter().position(|&n| n <= target)
    }
}
