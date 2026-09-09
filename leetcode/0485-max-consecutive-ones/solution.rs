impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut l = 0;
        for (r, &v) in nums.iter().enumerate() {
            if v != 1 {
                res = res.max(r - l);
                l = r + 1;
            } 
        }
        res = res.max(nums.len() - l);
        res as i32
    }
}
