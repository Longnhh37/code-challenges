impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if x == 0.0 {
            return if n == 0 { 1.0 } else { 0.0 };
        }

        let mut exp = n as i64;
        let invert = exp < 0;
        if invert {
            exp = -exp;
        }

        let mut res = 1.0_f64;
        let mut base = x;

        while exp > 0 {
            if exp & 1 == 1 {
                res *= base;
            }
            base *= base;
            exp >>= 1;
        }

        if invert {
            1.0 / res
        } else {
            res
        }
    }
}
