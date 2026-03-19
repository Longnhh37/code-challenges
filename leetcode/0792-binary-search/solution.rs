impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let i = nums.binary_search(&target).unwrap_or_else(|x| x);

        if i < nums.len() && target == nums[i] {
            return i as i32;
        }

        -1i32
    }
}
