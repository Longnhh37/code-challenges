impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut best = i32::MIN;
        let mut cur = 0;

        for &n in &nums {
            cur = n.max(cur + n);
            best = best.max(cur);
        }

        best
    }
}
