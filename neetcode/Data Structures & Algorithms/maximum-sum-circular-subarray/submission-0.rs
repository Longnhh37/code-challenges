impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let sum_all = nums.iter().sum::<i32>();
        let max_nonwrap = Self::max_subarray_sum(&nums);
        let max_wrap = sum_all - Self::min_subarray_sum(&nums);

        if max_nonwrap < 0 {
            max_nonwrap
        } else {
            max_nonwrap.max(max_wrap)
        }

    }

    fn max_subarray_sum(nums: &Vec<i32>) -> i32 {
        let mut max_sum = i32::MIN;
        let mut cur_sum = 0;

        for &n in nums {
            cur_sum += n;
            max_sum = max_sum.max(cur_sum);
            if cur_sum < 0 {
                cur_sum = 0;
            }
        }

        max_sum
    }

    fn min_subarray_sum(nums: &Vec<i32>) -> i32 {
        let mut min_sum = i32::MAX;
        let mut cur_sum = 0;

        for &n in nums {
            cur_sum += n;
            min_sum = min_sum.min(cur_sum);
            if cur_sum > 0 {
                cur_sum = 0;
            }
        }

        min_sum
    }
}
