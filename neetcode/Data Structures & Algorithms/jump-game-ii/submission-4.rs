impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let (mut l, mut r) = (0usize, 0usize);

        while r < nums.len() - 1 {
            let mut furthest = 0usize;
            for i in l..=r {
                furthest = furthest.max(i + nums[i] as usize);
            }
            (l, r) = (r + 1, furthest);
            res += 1
        }

        res
    }
}
