impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        nums.sort_unstable();

        let mut max_c = 0;
        let mut c = 1;
        let mut prev = nums[0];

        for &cur in nums.iter().skip(1) {
            if cur == prev + 1 {
                prev = cur;
                c += 1;
            } else if cur == prev {
                continue;
            } else {
                max_c = max_c.max(c);
                c = 1;
                prev = cur;
            }
        }

        max_c = max_c.max(c);
        max_c
    }
}
