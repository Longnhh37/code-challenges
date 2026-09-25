impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let (mut left_min, mut left_max) = (0i32, 0i32);

        for c in s.bytes() {
            match c {
                b'(' => {
                    left_min += 1;
                    left_max += 1;
                }
                b')' => {
                    left_min -= 1;
                    left_max -= 1;
                }
                _ => {
                    left_min -= 1;
                    left_max += 1;
                }
            }

            if left_max < 0 {
                return false;
            }
            if left_min < 0 {
                left_min = 0;
            }
        }
        left_min == 0
    }
}
