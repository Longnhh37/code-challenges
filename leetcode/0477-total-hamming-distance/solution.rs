impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i64;
        let mut total: i64 = 0;

        for bit in 0..32 {
            let ones: i64 = nums.iter()
                .filter(|&&x| (x >> bit) & 1 == 1)
                .count() as i64;
            let zeros = n - ones;
            total += ones * zeros;
        }

        total as i32
    }
}
