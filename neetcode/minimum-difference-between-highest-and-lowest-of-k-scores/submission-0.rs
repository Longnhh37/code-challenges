impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let k = k as usize;
        let mut res = i32::MAX;

        for w in nums.windows(k) {
            res = res.min(w[k - 1] - w[0]);
        }
        
        res
    }
}
