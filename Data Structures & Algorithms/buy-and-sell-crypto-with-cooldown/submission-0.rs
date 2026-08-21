use std::collections::HashMap;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut map: HashMap<(usize, bool), i32> = HashMap::new();
        Self::dfs(0, true, &prices, &mut map)
    }

    fn dfs(i: usize, buying: bool, prices: &[i32], map: &mut HashMap<(usize, bool), i32>) -> i32 {
        if i >= prices.len() {
            return 0;
        }
        if let Some(&profit) = map.get(&(i, buying)) {
            return profit;
        }
        let cooldown = Self::dfs(i + 1, buying, prices, map);

        let res = if buying {
            let buy = Self::dfs(i + 1, !buying, prices, map) - prices[i];
            cooldown.max(buy)
        } else {
            let sell = Self::dfs(i + 2, !buying, prices, map) + prices[i];
            cooldown.max(sell)
        };

        map.insert((i, buying), res);
        res
    }
}
