impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let mut n = nums.len();

        let mut prefix = vec![0];
        prefix.reserve(n);
        let mut sum = 0;
        for &n in &nums {
            sum += n;
            prefix.push(sum);
        }

        for i in 0..=n {
            for j in i + 1..=n {
                if prefix[j] - prefix[i] == k {
                    res += 1;
                }
            }
        }

        res
    }
}
