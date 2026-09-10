impl Solution {
    pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 1..n {
            if nums[i] < nums[i - 1] {
                if nums[i..].windows(2).any(|w| w[0] > w[1]) {
                    return -1;
                } else if nums[0] < nums[n - 1] {
                    return -1;
                } else {
                    return (n - i) as i32;
                }

            }
        }
        0
    }
}
