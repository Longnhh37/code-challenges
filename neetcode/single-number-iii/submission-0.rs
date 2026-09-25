impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut xor = 0;
        for &n in &nums {
            xor ^= n;
        }

        let mut diff_bit = 1;
        while xor & diff_bit == 0 {
            diff_bit <<= 1;
        }

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
