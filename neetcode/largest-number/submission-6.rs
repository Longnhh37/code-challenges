use std::cmp::Ordering;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        if nums.iter().all(|&n| n == 0) {
            return "0".to_string();
        }

        let mut strs: Vec<String> = nums.into_iter()
            .map(|n| n.to_string())
            .collect();
        
        strs.sort_unstable_by(|a, b| {
            let ab = format!("{a}{b}");
            let ba = format!("{b}{a}");
            ba.cmp(&ab)
        });

        strs.concat()
    }
}
