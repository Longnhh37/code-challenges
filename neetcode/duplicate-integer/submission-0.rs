impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut dup = std::collections::HashSet::new();
        for num in nums {
            if !dup.insert(num) {
                return true;
            }
        }

        false
    }
}
