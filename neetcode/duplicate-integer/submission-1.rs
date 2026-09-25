impl Solution {
    pub fn has_duplicate(mut nums: Vec<i32>) -> bool {
        nums.sort_unstable();
        nums.windows(2).any(|w| w[0] == w[1])
    }
}
