impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MIN;
        let mut min_dist = i32::MAX;

        for n in nums {
            let d = n.abs();
            if d < min_dist {
                min_dist = d;
                ans = n;
            } else if d == min_dist {
                ans = ans.max(n);
            }
        }

        ans
    }
}
