impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut write = 1;

        for read in 1..nums.len() {
            if write == 1 || nums[read] != nums[write - 2] {
                nums[write] = nums[read];
                write += 1;
            }
        }

        write as i32
    }
}
