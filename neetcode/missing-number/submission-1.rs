impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let missing = nums.iter().sum::<i32>();
        let full = n * (n + 1) / 2;
        full - missing
    }
}
