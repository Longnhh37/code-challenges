impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        if chars.is_empty() {
            return true;
        }
        let mut l = 0;
        let mut r = chars.len() - 1;

        while l < r {
            if chars[l] != chars[r] {
                return Self::is_pal(&chars, l + 1, r) || Self::is_pal(&chars, l, r - 1);
                }
            l += 1;
            r -= 1;
        }
        true
    }

    fn is_pal(chars: &[char], mut l: usize, mut r: usize) -> bool {
        while l < r {
            if chars[l] != chars[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}
