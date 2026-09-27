use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut deque = VecDeque::new();
        let mut res = Vec::with_capacity(nums.len() + 1 - k as usize);

        for i in 0..nums.len() {
            while let Some(&front) = deque.front() {
                if front + k <= i {
                    deque.pop_front();
                } else {
                    break;
                }
            }

            while let Some(&back) = deque.back() {
                if nums[back] <= nums[i] {
                    deque.pop_back();
                } else {
                    break;
                }
            }

            deque.push_back(i);

            if i + 1 >= k {
                res.push(nums[*deque.front().unwrap()]);
            }
        }

        res
    }
}
