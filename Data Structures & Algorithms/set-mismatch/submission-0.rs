impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i64;
        let mut x = 0i64;
        let mut y = 0i64;

        for i in 1..=n {
            x += nums[(i - 1) as usize] as i64 - i;
            y += (nums[(i - 1) as usize] as i64).pow(2) - i * i;
        }

        let d = (x + y / x) / 2;
        let m = d - x;

        vec![d as i32, m as i32]
    }
}