impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut rev = 0i32;

        while x != 0 {
            let digit = x % 10;
            x /= 10;

            rev = match rev.checked_mul(10) {
                None => return 0,
                Some(v) => v,
            };

            rev = match rev.checked_add(digit) {
                None => return 0,
                Some(v) => v,
            };
       }

        rev
    }
}
