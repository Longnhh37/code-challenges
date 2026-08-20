impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut destination = nums.len() - 1;
        for (i, &n) in nums.iter().enumerate().rev() {
            let n = n as usize;
            if i + n >= destination {
                destination = i;
            }
        }

        destination == 0
    }
}
