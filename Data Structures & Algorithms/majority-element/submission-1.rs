impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut cur = nums[0];
        let mut count = 1;

        for num in nums.into_iter().skip(1) {
            if num != cur {
                count -= 1;
            } else {
                count += 1;
            }
            if count == 0 {
                cur = num;
                count += 1;
            }
        }

        cur
    }
}
