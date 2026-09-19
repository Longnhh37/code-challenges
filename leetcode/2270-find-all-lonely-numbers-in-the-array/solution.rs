use std::collections::HashMap;

impl Solution {
    pub fn find_lonely(nums: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        for &n in &nums {
            *map.entry(n).or_insert(0u32) += 1;
        }

        let mut res = Vec::new();

        for n in nums {
            if map.get(&n) == Some(&1)
                && map.get(&(n - 1)).is_none()
                && map.get(&(n + 1)).is_none() 
            {
                res.push(n);
            }
        }

        res
    }
}
