use std::collections::HashSet;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut seen = HashSet::new();       
        loop {
            let mut sum = 0;
            while n > 0 {
                sum += (n % 10).pow(2);
                n /= 10;
            }
            if sum == 1 {
                return true;
            }
            if !seen.insert(sum) {
                return false;
            }
            n = sum;
        }
    }
}
