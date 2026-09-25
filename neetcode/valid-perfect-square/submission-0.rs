impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut i = 1;
        while i * i <= num {
            if i * i == num {
                return true;
            }
            i += 1;
        }
        false
    }
}
