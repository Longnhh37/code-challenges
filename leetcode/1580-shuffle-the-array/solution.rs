impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let n2 = n * 2;
        let mut res = Vec::with_capacity(2 * n);
        let mut i = 0;
        for _ in 0..2 * n {
            res.push(nums[i]);
            i += n;
            if i >= n2 {
                i = i % n2 + 1;
            }
        }

        res
    }
}
