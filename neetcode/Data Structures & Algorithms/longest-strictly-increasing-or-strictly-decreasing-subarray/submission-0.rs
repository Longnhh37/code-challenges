impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 1;
        }

        let mut res = i32::MIN;
        let (mut l1, mut r1) = (0, 1);
        let (mut l2, mut r2) = (0, 1);

        while r1 < n {
            if nums[r1] > nums[r1 - 1] {
                res = res.max((r1 - l1 + 1) as i32);
            } else {
                l1 = r1;
            }

            if nums[r2] < nums[r2 - 1] {
                res = res.max((r2 - l2 + 1) as i32);
            } else {
                l2 = r2;
            }

            r1 += 1;
            r2 += 1;
        }


        if res == i32::MIN { 1 } else { res }
    }
}
