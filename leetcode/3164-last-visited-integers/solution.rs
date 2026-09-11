use std::collections::VecDeque;

impl Solution {
    pub fn last_visited_integers(nums: Vec<i32>) -> Vec<i32> {
        let mut seen = VecDeque::new();
        let mut ans = Vec::new();
        let mut cnt = 0;

        for &n in &nums {
            if n != -1 {
                seen.push_front(n);
                cnt = 0;
            } else {
                cnt += 1;
                if cnt <= seen.len() {
                    ans.push(seen[cnt - 1]);
                } else {
                    ans.push(-1);
                }
            }
        }       

        ans
    }
}
