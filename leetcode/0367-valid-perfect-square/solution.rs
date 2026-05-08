impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut l = 1;
        let mut r = num;

        while l <= r {
            let mid = l + (r - l) / 2;

            if mid == num / mid && num % mid == 0 {
                return true;
            } else if mid < num / mid {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        false
    }
}
