impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        for _ in 0..k {
            *nums.iter_mut().min().unwrap() *= multiplier;
        }
        nums
    }
}
