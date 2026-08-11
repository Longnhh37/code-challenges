impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut l = 0;
        for r in 0..nums.len() {
            if nums[r] % 2 == 0 {
                nums.swap(l, r);
                l += 1;
            }
        }
        nums
    }
}
