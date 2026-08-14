impl Solution {
    pub fn number_of_matches(mut n: i32) -> i32 {
        let mut res = 0;
        while n > 1 {
            res += n / 2;
            n -= n / 2;
        }
        res
    }
}
