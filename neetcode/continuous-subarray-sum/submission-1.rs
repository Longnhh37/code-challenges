impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut pref = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            pref[i + 1] = pref[i] + nums[i];
        }
        for size in 2..=nums.len() {
            for i in size..=nums.len() {
                let total = pref[i] - pref[i - size];
                if total % k == 0 {
                    return true;
                }
            }
        }

        false
    }
}
