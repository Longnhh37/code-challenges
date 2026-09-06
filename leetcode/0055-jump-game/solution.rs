impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut end = nums.len() - 1;
        for (i, &n) in nums.iter().enumerate().rev() {
            let n = n as usize;
            if i + n >= end {
                end = i;
            }
        }

        end == 0
    }
}
