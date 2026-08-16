impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = std::collections::HashMap::new();
        dp.insert(0, 1);

        for total in 1..=target {
            dp.insert(total, 0);
            for &n in &nums {
                let prev = *dp.get(&(total - n)).unwrap_or(&0);
                *dp.get_mut(&total).unwrap() += prev;
            }
        }

        *dp.get(&target).unwrap()
    }
}
