impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut l = 0;
        let mut res = 0;

        for r in 1..n {
            let left = prices[l];
            let right = prices[r];

            if left < right {
                res = res.max(right - left);
            } else {
                l = r;
            }
        }
        res
    }
}
