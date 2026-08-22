impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut max_cnt = 0;

        for n in nums {
            if n == 1 {
                cnt += 1;
            } else {
                max_cnt = max_cnt.max(cnt);
                cnt = 0;
            }
        }

        max_cnt.max(cnt)
    }
}
