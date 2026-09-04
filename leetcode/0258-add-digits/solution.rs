impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        while num >= 10 {
            let mut cur = 0;
            while num > 0 {
                cur += num % 10;
                num /= 10;
            }
            num = cur;
        }
        num
    }
}
