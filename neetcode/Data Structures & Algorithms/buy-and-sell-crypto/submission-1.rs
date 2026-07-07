impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut max_profit = 0;

        for &p in &prices {
            min_price = min_price.min(p);
            max_profit = max_profit.max(p - min_price);
        }

        max_profit
    }
}