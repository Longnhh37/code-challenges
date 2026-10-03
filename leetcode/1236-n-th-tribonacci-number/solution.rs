impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let (mut a, mut b, mut c) = (0, 1, 1);
        for _ in 2..n {
            (a, b, c) = (b, c, a + b + c);
        }

        c
    }
}
