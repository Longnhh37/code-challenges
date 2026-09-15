impl Solution {
    pub fn smallest_number(mut n: i32, t: i32) -> i32 {
        while Self::get_prod(n) % t != 0 {
            n += 1;
        }
        n
    }

    fn get_prod(mut n: i32) -> i32 {
        let mut res = 1;
        while n > 0 {
            res *= n % 10;
            n /= 10;
        }
        res
    }
}
