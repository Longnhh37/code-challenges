use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut dq = VecDeque::new();
        let mut res = Vec::new();

        for i in 0..nums.len() {
            while let Some(&front) = dq.front() {
                if front + k <= i {
                    dq.pop_front();
                } else {
                    break;
                }
            }

            while let Some(&back) = dq.back() {
                if nums[back] <= nums[i] {
                    dq.pop_back();
                } else {
                    break;
                }
            }

            dq.push_back(i);
            if i + 1 >= k {
                res.push(nums[*dq.front().unwrap()]);
            }
        }

        res
    }
}
