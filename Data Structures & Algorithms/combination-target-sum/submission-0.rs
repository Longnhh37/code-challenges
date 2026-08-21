impl Solution {
    pub fn combination_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut path = Vec::new();
        Self::backtrack(0, target, &nums, &mut path, &mut res);
        res


    }

    fn backtrack(
        start: usize,
        remaining: i32,
        nums: &[i32],
        path: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {

    if remaining == 0 {
        return res.push(path.clone());
    }

    if remaining < 0 {
        return;
    }

    for i in start..nums.len() {
        path.push(nums[i]);
        Self::backtrack(
            i,
            remaining - nums[i],
            nums,
            path,
            res,
        );
        path.pop();
    }
}

}