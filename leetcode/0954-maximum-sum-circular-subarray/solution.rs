impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut total = 0;
        let mut max_ending_here = 0;
        let mut min_ending_here = 0;
        let mut max_sum = nums[0];
        let mut min_sum = nums[0];

        for &n in &nums {
            total += n;

            max_ending_here = n.max(max_ending_here + n);
            max_sum = max_sum.max(max_ending_here);

            min_ending_here = n.min(min_ending_here + n);
            min_sum = min_sum.min(min_ending_here);
        }
        
        if max_sum < 0 {
            max_sum
        } else {
            max_sum.max(total - min_sum)
        }
    }
}
