impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
       let mut l = 0;

        for r in 0..nums.len() {
            if nums[r] != 0 {
                nums[l] = nums[r];
                l += 1;
            }
        }

        for i in l..nums.len() {
            nums[i]=0;
        }
    }
}

