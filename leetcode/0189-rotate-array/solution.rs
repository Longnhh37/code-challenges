impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = k as usize;
        let i = n - (k % n);

        let mut tmp = nums[i..].to_vec();
        tmp.extend_from_slice(&nums[..i]);

        std::mem::swap(nums, &mut tmp);
    }
}
