impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut prefix = vec![0; nums.len() + 1];
        
        for (i, &n) in nums.iter().enumerate() {
            prefix[i + 1] = prefix[i] + n;
        }

        let mut res = i32::MIN;
        let mut l = 0;
        for r in 1..nums.len() {
            if nums[r] > nums[r - 1] {
                res = res.max(prefix[r + 1] - prefix[l]);
            } else {
                l = r;
            }
        }

        if res == i32::MIN { prefix[1] } else { res }
    }
}
