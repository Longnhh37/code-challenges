impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();

        while l < r {
            if nums[l] == val {
                nums.swap(l, r - 1);
                r -= 1;
            } else {
                l += 1;
            }
        }

        r as i32
    }
}
