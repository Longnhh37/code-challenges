impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if x == 0.0 {
            return x;
        }

        let invert: bool;
        let n = if n >= 0 { 
            invert = false;
            n
        } else {
            invert = true;
            -n
        };
        let mut res = 1.0_f64;

        for _ in 0..n {
            res *= x;
        }

        if invert {
            1.0 / res
        } else {
            res
        }
    }
}
