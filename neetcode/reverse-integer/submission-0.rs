impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x as i64;
        let mut rev = 0i64;
        while x != 0 {
            rev = rev * 10 + x % 10;
            x /= 10;
        }

        i32::try_from(rev).unwrap_or(0)
    }
}
