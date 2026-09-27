impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_end = nums[0];
        let mut min_end = nums[0];
        let mut best = nums[0];
        
        for &n in &nums[1..] {
            let candidates = [n, max_end * n, min_end * n];
            max_end = *candidates.iter().max().unwrap();
            min_end = *candidates.iter().min().unwrap();

            best = best.max(max_end);
        }

        best
    }
}
