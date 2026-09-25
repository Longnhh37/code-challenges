impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amount = amount as usize;
        let mut prev = vec![0; amount + 1];
        prev[0] = 1;

        for (i, &c) in coins.iter().enumerate().rev() {
            let mut cur = vec![0; amount + 1];
            cur[0] = 1;

            for a in 1..=amount {
                cur[a] = prev[a];
                if a >= coins[i] as usize {
                    cur[a] += cur[a - coins[i] as usize];
                }
            }
            prev = cur;
        }

        prev[amount]
    }
}
