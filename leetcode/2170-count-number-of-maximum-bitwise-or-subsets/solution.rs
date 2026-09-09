impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let max = nums.iter().fold(0i32, |acc, &x| acc | x);
        let mut res = 0;
        Self::backtrack(&nums, 0, max, &mut 0, &mut res);
        res
    }

    fn backtrack(nums: &[i32], i: usize, max: i32, path: &mut i32, res: &mut i32) {
        if i == nums.len() {
            if *path == max {
                *res += 1;
            }
            return;
        }
        let cur = nums[i];
        let old = *path;

        // pick cur
        *path |= cur;
        Self::backtrack(nums, i + 1, max, path, res);
        *path = old;

        // skip cur
        Self::backtrack(nums, i + 1, max, path, res);
    }
}
