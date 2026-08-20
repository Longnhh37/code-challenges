impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut zero, mut one) = (0, 0);
        for two in 0..nums.len() {
            let tmp = nums[two];
            nums[two] = 2;
            if tmp < 2 {
                nums[one] = 1;
                one += 1;
            }
            if tmp < 1 {
                nums[zero] = 0;
                zero += 1;
            }
        }
    }
}