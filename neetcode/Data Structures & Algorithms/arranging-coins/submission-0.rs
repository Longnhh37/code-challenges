impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let n = n as i64;
        let (mut l, mut r) = (1i64, n as i64);

        while l <= r {
            let m = l + (r - l) / 2;
            let sum = (m + 1) * m / 2;

            if sum < n {
                l = m + 1;
            } else if sum > n {
                r = m - 1;
            } else {
                return m as i32;
            }
        }

        (l + (r - l) / 2 - 1) as i32
    }
}
