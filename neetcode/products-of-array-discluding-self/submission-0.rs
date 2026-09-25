impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut ans = vec![1; len];

        for i in 1..len {
            ans[i] = ans[i - 1] * nums[i - 1];
        }

        let mut right_prod = 1;
        for j in (0..len).rev() {
            ans[j] *= right_prod;
            right_prod *= nums[j];
        }

        ans
    }
}
