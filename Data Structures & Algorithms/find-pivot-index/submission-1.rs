impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.iter().sum::<i32>();

        for i in 0..nums.len() {
            right -= nums[i];
            if left == right {
                return i as i32;
            }
            left += nums[i];
        }
        
        -1
    }
}
