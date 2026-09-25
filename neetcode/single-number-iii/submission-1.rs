impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut xor = 0;
        for &n in &nums {
            xor ^= n;
        }
        let diff_bit = xor & (-xor);

        let (mut a, mut b) = (0, 0);
        for &n in &nums {
            if n & diff_bit != 0 {
                a ^= n;
            } else {
                b ^= n;
            }
        }

        vec![a, b]
    }
}
