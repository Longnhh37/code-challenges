impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let len = nums.len();

        if len == 1 || nums[0] > nums[1] {
            return 0;
        }
        if nums[len - 2] < nums[len - 1] {
            return len as i32 - 1;
        }

        for i in 1..nums.len() - 1 {
            if nums[i - 1] < nums[i] && nums[i] > nums[i + 1] {
                return i as i32;
            }
        }

        unreachable!()
    }
}
