impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let chars = s.into_bytes();
        let (l, r) = Self::expand(&chars, 0, chars.len() - 1);
        if l >= r {
            return true;
        } else {
            let (l1, r1) = Self::expand(&chars, l + 1, r);
            let (l2, r2) = Self::expand(&chars, l, r - 1);
            if l1 >= r1 || l2 >= r2 {
                return true;
            }
        }

        false
    }

    fn expand(chars: &[u8], mut l: usize, mut r: usize) -> (usize, usize) {
        while l < r {
            if chars[l] == chars[r] {
                l += 1;
                r -= 1;
            } else {
                break;
            }
        }
        (l, r)
    }
}
