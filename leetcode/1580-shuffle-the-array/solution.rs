impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut res = vec![0; n * 2];
        let (mut x, mut y) = (0, n);
        let mut i = 0;

        for _ in 0..n {
            res[i] = nums[x];
            res[i + 1] = nums[y];
            x += 1;
            y += 1;
            i += 2;
        }

        res
    }
}
