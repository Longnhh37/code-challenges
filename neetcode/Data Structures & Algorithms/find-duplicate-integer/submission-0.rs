impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = 0usize;
        let mut fast = 0usize;
        loop {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
            if slow == fast {
                break;
            }
        }

        let mut slow2 = 0usize;
        loop {
            slow = nums[slow] as usize;
            slow2 = nums[slow2] as usize;
            if slow == slow2 {
                return slow as i32;
            }
        }
    }
}
