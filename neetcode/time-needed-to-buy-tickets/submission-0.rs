impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut res = 0;
        for i in 0..tickets.len() {
            if i <= k {
                res += tickets[i].min(tickets[k]);
            } else {
                res += tickets[i].min(tickets[k] - 1);
            }
        }
         res
    }
}