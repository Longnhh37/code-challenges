impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![-1; n];
        let mut stack = Vec::new();

        for i in (0..2 * n).rev() {
            let idx = i % n;
            let n_val = nums[idx];

            while let Some(&top) = stack.last() {
                if top <= n_val {
                    stack.pop();
                } else {
                    break;
                }
            }
            
                res[idx] = stack.last().copied().unwrap_or(-1);
            stack.push(n_val);
        }

        res
    }
}
