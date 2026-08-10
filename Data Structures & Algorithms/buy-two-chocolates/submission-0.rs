impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let (i, min1) = prices
            .iter()
            .enumerate()
            .min_by_key(|&(_, n)| n)
            .unwrap();
        
        let (_, min2) = prices
            .iter()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .min_by_key(|&(_, n)| n)
            .unwrap();

        if min1 + min2 > money {
            money
        } else {
            money - min1 - min2
        }
    }
}
