impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut shortest = i32::MAX;
        let mut l = 0;
        let mut sum = 0;

        for r in 0..nums.len() {
            sum += nums[r];
            while sum >= target {
                shortest = shortest.min((r - l + 1) as i32);
                sum -= nums[l];
                l += 1;
            }
        }

        if shortest == i32::MAX { 0 } else { shortest }
    }
}
