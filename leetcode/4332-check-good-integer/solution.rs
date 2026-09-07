impl Solution {
    pub fn check_good_integer(mut n: i32) -> bool {
        let mut sq = 0;
        let mut sum = 0;
        while n > 0 {
            let d = n % 10;
            sq += d * d;
            sum += d;
            n /= 10;
        }
        sq - sum >= 50
    }
}
