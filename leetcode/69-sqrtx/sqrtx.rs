impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as i64;
        let (mut l, mut r) = (0i64, x);
        while l <= r {
            let mid = l + (r - l) / 2;
            let sq = mid * mid;
            if sq == x {
                return mid as i32;
            } else if sq < x {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        (l - 1) as i32
    }
}