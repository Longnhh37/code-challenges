impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = nums.len();
        let mut pref = vec![0i64; n + 1];
        for i in 1..n + 1 {
            pref[i] = pref[i - 1] + nums[i - 1] as i64;
        }
        for i in 1..n {
            res += (pref[i] >= pref[n] - pref[i]) as i32;
        }
        res
        // 0 2 5 6 6
    }
}
