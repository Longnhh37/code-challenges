impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let s = x.to_string();
        let b = s.as_bytes();
        let (mut l, mut r) = (0, b.len() - 1);

        while l < r {
            if b[l] != b[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }

        true
    }
}
