impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.partition_point(|&v| v < target) as i32
    }
}
