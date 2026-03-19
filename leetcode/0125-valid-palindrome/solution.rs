impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let bytes = s.as_bytes();
    
        if bytes.is_empty() {
            return true;
        }

        let mut l = 0;
        let mut r = bytes.len() - 1;

        while l < r {
            while l < r && !bytes[l].is_ascii_alphanumeric() {
                l += 1;
            }
            while l < r && !bytes[r].is_ascii_alphanumeric() {
                r -= 1;
            }

            if l < r { 
                if bytes[l].to_ascii_lowercase() != bytes[r].to_ascii_lowercase() {
                    return false;
                }

                l += 1;
                r -= 1;
            }
        }

        true
    }
}
