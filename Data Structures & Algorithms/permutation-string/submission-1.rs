impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut c1 = [0; 26];
        for b in s1.bytes() {
            c1[(b - b'a') as usize] += 1;
        }

        let b2 = s2.as_bytes();
        if b2.len() < s1.len() {
            return false;
        }

        let mut c2 = [0; 26];
        let (mut l, mut r) = (0, 0);

        for _ in 0..s1.len() {
            c2[(b2[r] - b'a') as usize] += 1;
            r += 1;
        }
        r -= 1;
        if c1 == c2 {
            return true;
        }

        for _ in 0..b2.len() - s1.len() {
            l += 1;
            r += 1;
            c2[(b2[l - 1] - b'a') as usize] -= 1;
            c2[(b2[r] - b'a') as usize] += 1;
            if c1 == c2 {
                return true;
            }
        }

        false
    }
}
