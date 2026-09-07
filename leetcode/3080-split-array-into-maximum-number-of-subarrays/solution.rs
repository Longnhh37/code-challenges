impl Solution {
    pub fn max_subarrays(nums: Vec<i32>) -> i32 {
        let total_and = nums.iter().fold(!0i32, |acc, &x| acc & x);
        if total_and != 0 {
            return 1;
        }

        let mut cnt = 0;
        let mut running_and = !0i32;

        for &n in &nums {
            running_and &= n;
            if running_and == 0 {
                cnt += 1;
                running_and = !0i32;
            }
        }

        cnt
    }
}
