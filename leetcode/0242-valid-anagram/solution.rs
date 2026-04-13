impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut counter = [0; 26];

        for b in s.bytes() {
            counter[(b - b'a') as usize] += 1;
        }

        for b in t.bytes() {
            let i = (b - b'a') as usize;
            if counter[i] == 0 {
                return false;
            }
            counter[i] -= 1;
        }

        !counter.iter().any(|&v| v != 0)
    }
}

