impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let mut tmp = n;
        let mut sum = 0;
        let mut prod = 1;

        while tmp > 0 {
            let d = tmp % 10;
            sum += d;
            prod *= d;
            tmp /= 10;
        }

        n % (sum + prod) == 0
    }
}
