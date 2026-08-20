impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let (mut fast, mut slow) = (n, n);
        loop {
            fast = Self::sum_square_digits(fast);
            fast = Self::sum_square_digits(fast);

            if fast == 1 {
                return true;
            }

            slow = Self::sum_square_digits(slow);

            if fast == slow {
                return false;
            }
        }
    }

    fn sum_square_digits(mut n: i32) -> i32 {
        let mut res = 0;
        while n > 0 {
            res += (n % 10).pow(2);
            n /= 10;
        }
        res
    }
}
