impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let weeks = n / 7;
        let days = n - weeks * 7;
        let mut res = 0;

        for w in 1..=weeks {
            res += 28 + 7 * (w - 1);
        }

        let last_mon = weeks + 1;
        res += (2 * last_mon + days - 1) * days / 2;
        res
    }
}
