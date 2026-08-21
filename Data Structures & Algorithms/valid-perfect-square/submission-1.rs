impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let (mut l, mut r) = (1i64, num as i64);
        let target = num as i64;

        while l <= r {
            let mid = l + (r - l) / 2;
            let square = mid * mid;

            if square == target {
                return true;
            } else if square < target {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        false
    }
}
