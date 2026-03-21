impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 3 {
            return n;
        }

          let (mut a, mut b) = (2, 3);
        for _ in 4..=n {
            (a, b) = (b, a + b)
        }

        b
    }
}

