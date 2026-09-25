impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        let (mut i, mut j) = (0, 1);

        for &n in &nums {
            if n > 0 {
                res[i] = n;
                i += 2;
            } else {
                res[j] = n;
                j += 2;
            }
        }

        res
    }
}
