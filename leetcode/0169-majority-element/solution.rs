impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut cnt = 0;

        for n in nums {
            if cnt == 0 {
                res = n;
            }
            cnt += if n == res { 1 } else { -1 };
        }

        res
    }
}
